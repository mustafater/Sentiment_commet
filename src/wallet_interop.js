let _api = null;

async function loadScript(src) {
    return new Promise((resolve, reject) => {
        if (document.querySelector(`script[src="${src}"]`)) {
            // Already appended, check if loaded?
            if (window.StellarSdk) return resolve();
            // If not loaded yet, wait a bit or attach listener? 
            // Simplified: just wait
            let attempts = 0;
            const interval = setInterval(() => {
                if (window.StellarSdk) {
                    clearInterval(interval);
                    resolve();
                }
                attempts++;
                if (attempts > 50) { // 5s
                    clearInterval(interval);
                    reject(new Error('Script loaded but SDK missing'));
                }
            }, 100);
            return;
        }

        console.log(`[Wallet] Injecting script: ${src}`);
        const script = document.createElement('script');
        script.src = src;
        script.type = 'text/javascript';
        script.async = true;

        script.onload = () => {
            console.log(`[Wallet] Script loaded: ${src}`);
            if (window.StellarSdk) resolve();
            else reject(new Error('StellarSdk not found in window after load'));
        };
        script.onerror = (e) => {
            console.error(`[Wallet] Script load error: ${src}`, e);
            reject(new Error(`Failed to load ${src}`));
        };

        document.head.appendChild(script);
    });
}

async function getApi() {
    if (_api) return _api;
    try {
        // Load Freighter API - simplified to use window.freighter directly if possible
        let freighter = window.freighter;

        // If not available, try to import (maybe it injects itself?)
        if (!freighter) {
            try {
                const mod = await import('https://esm.sh/@stellar/freighter-api@3.0.0');
                freighter = {
                    isConnected: mod.isConnected || (mod.default && mod.default.isConnected),
                    requestAccess: mod.requestAccess || (mod.default && mod.default.requestAccess),
                    getAddress: mod.getAddress || (mod.default && mod.default.getAddress),
                    signTransaction: mod.signTransaction || (mod.default && mod.default.signTransaction),
                };
            } catch (e) {
                console.warn('[Wallet] Failed to load @stellar/freighter-api module', e);
            }
        }

        // Dynamically load StellarSdk UMD
        if (!window.StellarSdk) {
            try {
                await loadScript('https://unpkg.com/@stellar/stellar-sdk@12.3.0/dist/stellar-sdk.min.js');
            } catch (e) {
                console.error('[Wallet] Unpkg failed, trying jsdelivr fallback');
                await loadScript('https://cdn.jsdelivr.net/npm/@stellar/stellar-sdk@12.3.0/dist/stellar-sdk.min.js');
            }
        }

        const StellarSdk = window.StellarSdk;

        if (!StellarSdk) {
            console.error('[Wallet] Failed to load StellarSdk (window.StellarSdk is null)');
            throw new Error('StellarSdk load failed');
        }

        console.log('[Wallet] StellarSdk loaded. Keys:', Object.keys(StellarSdk));

        _api = {
            isConnected: freighter ? freighter.isConnected : () => Promise.resolve(false),
            requestAccess: freighter ? freighter.requestAccess : () => Promise.resolve({ error: 'Freighter not found' }),
            getAddress: freighter ? freighter.getAddress : () => Promise.resolve({ error: 'Freighter not found' }),
            signTransaction: freighter ? freighter.signTransaction : () => Promise.resolve({ error: 'Freighter not found' }),
            StellarSdk: StellarSdk,
        };
        return _api;
    } catch (e) {
        console.error('[Wallet] Failed to load APIs:', e);
        return null;
    }
}

export async function freighter_connect() {
    try {
        const api = await getApi();
        if (!api || !api.isConnected) {
            alert('Failed to load Freighter API. Please make sure the extension is installed.');
            return '';
        }

        const connResult = await api.isConnected();
        if (!connResult || !connResult.isConnected) {
            alert('Freighter wallet not detected or not connected.\\nInstall from https://freighter.app and reload.');
            return '';
        }

        const accessResult = await api.requestAccess();
        if (accessResult.error) {
            console.error('[Wallet] Access denied:', accessResult.error);
            alert('Freighter access denied: ' + accessResult.error);
            return '';
        }

        return accessResult.address || '';
    } catch (e) {
        console.error('[Wallet] Connect error:', e);
        return '';
    }
}

export async function freighter_get_public_key() {
    try {
        const api = await getApi();
        if (!api || !api.getAddress) return '';
        const result = await api.getAddress();
        if (result.error) return '';
        return result.address || '';
    } catch (e) {
        console.error('[Wallet] getAddress error:', e);
        return '';
    }
}

export async function freighter_is_connected() {
    try {
        const api = await getApi();
        if (!api || !api.isConnected) return false;
        const result = await api.isConnected();
        return result && result.isConnected ? true : false;
    } catch (e) {
        return false;
    }
}

export async function freighter_sign_tx(xdr, network) {
    try {
        const api = await getApi();
        if (!api || !api.signTransaction) return '';

        const np = network === 'TESTNET'
            ? 'Test SDF Network ; September 2015'
            : 'Public Global Stellar Network ; September 2015';

        const result = await api.signTransaction(xdr, { networkPassphrase: np });
        if (result.error) return '';
        return result.signedTxXdr || '';
    } catch (e) {
        console.error('[Wallet] Sign error:', e);
        return '';
    }
}

export async function submit_negative_comment_js(
    rpcUrl,
    networkPassphrase,
    contractId,
    commentId,
    score,
    contentHash,
    publicKey
) {
    console.log('--- STARTING SUBMISSION (JS/DynamicInjector) ---');
    try {
        const api = await getApi();
        if (!api) return 'Failed to load SDK';

        const { StellarSdk } = api;

        // Ensure we have what we need
        if (!StellarSdk.TransactionBuilder) return 'Error: TransactionBuilder missing';
        if (!StellarSdk.SorobanRpc) return 'Error: SorobanRpc missing';

        console.log(`[Wallet] Received publicKey: '${publicKey}'`);

        if (typeof publicKey === 'string') publicKey = publicKey.trim();

        const { Contract, TransactionBuilder, SorobanRpc, xdr } = StellarSdk;
        const server = new SorobanRpc.Server(rpcUrl);

        const accountData = await server.getAccount(publicKey);
        if (!accountData) return `Account ${publicKey} not found.`;

        // Account creation
        let account = accountData;
        try {
            let seq = "0";
            if (accountData.sequence !== undefined) seq = String(accountData.sequence);
            else if (accountData.seqNum !== undefined) seq = String(accountData.seqNum);

            account = new StellarSdk.Account(publicKey, seq);
        } catch (e) {
            console.error('[Wallet] Account creation failed:', e);
        }

        const contract = new Contract(contractId);
        const args = [
            xdr.ScVal.scvString(commentId),
            xdr.ScVal.scvU32(score),
            xdr.ScVal.scvString(contentHash)
        ];

        const op = contract.call("submit_negative", ...args);

        const txBuilder = new TransactionBuilder(account, { fee: "1000", networkPassphrase });
        let tx = txBuilder
            .addOperation(op)
            .setTimeout(30)
            .build();

        console.log('[Wallet] Transaction built. Type:', tx.constructor.name);

        // Immediate check before even preparing
        if (typeof tx.toXDR !== 'function') {
            console.error('[Wallet] ALARM: tx.toXDR is missing on built transaction!');
            // Attempt to unwrap if it's wrapped?
        }

        // Prepare
        try {
            console.log('[Wallet] Preparing transaction...');
            tx = await server.prepareTransaction(tx);
            console.log('[Wallet] Prepared. Type:', tx.constructor.name);
        } catch (e) {
            console.error('[Wallet] prepareTransaction failed:', e);
            // Don't fail immediately, maybe simpler submission works?
            // return 'Error: Simulation failed: ' + e.message;
        }

        let xdrStr;
        if (typeof tx.toXDR === 'function') {
            try {
                xdrStr = tx.toXDR();
            } catch (e) {
                console.error('[Wallet] toXDR failed:', e);
            }
        }

        // Fallback
        if (!xdrStr && tx.toEnvelope) {
            try {
                xdrStr = tx.toEnvelope().toXDR('base64');
            } catch (e) { console.error('[Wallet] toEnvelope failed:', e); }
        }

        if (!xdrStr) {
            // LAST RESORT: Manual XDR construction? 
            // Too complex to inline here.
            return 'Error: XDR Serialization Failed. Object keys: ' + Object.keys(tx).join(',');
        }

        console.log('[Wallet] XDR generated:', xdrStr);

        const signedRes = await api.signTransaction(xdrStr, { networkPassphrase });
        if (signedRes.error) return 'Freighter sign error: ' + signedRes.error;

        const signedXdr = signedRes.signedTxXdr || signedRes;

        console.log('[Wallet] Signed XDR received. Parsing back to Transaction object...');
        // sendTransaction expects a Transaction object, not a string!
        let signedTx;
        try {
            signedTx = TransactionBuilder.fromXDR(signedXdr, networkPassphrase);
        } catch (e) {
            console.error('[Wallet] Failed to parse signed XDR:', e);
            return 'Error: Could not parse signed transaction';
        }

        const result = await server.sendTransaction(signedTx);

        if (result.status !== "PENDING" && result.status !== "SUCCESS") {
            return 'Submission failed: ' + JSON.stringify(result);
        }

        return 'SUCCESS:' + result.hash;
    } catch (e) {
        console.error(e);
        return 'Error: ' + e.message;
    }
}
