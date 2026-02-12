// Freighter Wallet JS Bridge
// Provides window-level functions for WASM to call via wasm_bindgen.
// Supports both modern @stellar/freighter-api and legacy window.freighter.

(function () {
    'use strict';

    // Helper: detect Freighter extension
    function getFreighterApi() {
        // Modern Freighter (v5+) uses window.freighterApi
        if (typeof window.freighterApi !== 'undefined') {
            return window.freighterApi;
        }
        // Legacy Freighter uses window.freighter
        if (typeof window.freighter !== 'undefined') {
            return window.freighter;
        }
        return null;
    }

    // Wait for Freighter to load (it injects after DOM ready)
    function waitForFreighter(timeout) {
        return new Promise((resolve) => {
            const api = getFreighterApi();
            if (api) { resolve(api); return; }

            const start = Date.now();
            const interval = setInterval(() => {
                const api = getFreighterApi();
                if (api) {
                    clearInterval(interval);
                    resolve(api);
                } else if (Date.now() - start > timeout) {
                    clearInterval(interval);
                    resolve(null);
                }
            }, 200);
        });
    }

    window.freighterConnect = async function () {
        try {
            const api = await waitForFreighter(3000);
            if (!api) {
                console.warn('[Wallet] Freighter not detected. Install from https://freighter.app');
                alert('Freighter wallet extension not detected.\n\nPlease install it from https://freighter.app and reload the page.');
                return '';
            }

            // Request access / permission
            if (typeof api.requestAccess === 'function') {
                await api.requestAccess();
            } else if (typeof api.isConnected === 'function') {
                const connected = await api.isConnected();
                if (!connected) {
                    console.warn('[Wallet] Freighter is installed but not connected');
                }
            }

            // Get public key
            let publicKey = '';
            if (typeof api.getPublicKey === 'function') {
                const result = await api.getPublicKey();
                publicKey = typeof result === 'string' ? result : (result && result.publicKey ? result.publicKey : '');
            } else if (typeof api.getAddress === 'function') {
                // Newer Freighter API uses getAddress
                const result = await api.getAddress();
                publicKey = typeof result === 'string' ? result : (result && result.address ? result.address : '');
            }

            if (publicKey) {
                console.log('[Wallet] Connected:', publicKey.substring(0, 6) + '...' + publicKey.substring(publicKey.length - 4));
            } else {
                console.warn('[Wallet] Connected but no public key returned');
            }
            return publicKey;
        } catch (error) {
            console.error('[Wallet] Connect error:', error);
            return '';
        }
    };

    window.freighterGetPublicKey = async function () {
        try {
            const api = getFreighterApi();
            if (!api) return '';

            if (typeof api.getPublicKey === 'function') {
                const result = await api.getPublicKey();
                return typeof result === 'string' ? result : (result && result.publicKey ? result.publicKey : '');
            }
            if (typeof api.getAddress === 'function') {
                const result = await api.getAddress();
                return typeof result === 'string' ? result : (result && result.address ? result.address : '');
            }
            return '';
        } catch (error) {
            console.error('[Wallet] getPublicKey error:', error);
            return '';
        }
    };

    window.freighterIsConnected = function () {
        try {
            const api = getFreighterApi();
            if (!api) return false;
            if (typeof api.isConnected === 'function') {
                const result = api.isConnected();
                // isConnected may return a boolean or a Promise or an object
                if (typeof result === 'boolean') return result;
                if (result && typeof result.isConnected === 'boolean') return result.isConnected;
                // If it returns a Promise, we can't await here, so return true (extension exists)
                return true;
            }
            // Extension exists if we got here
            return true;
        } catch (error) {
            return false;
        }
    };

    window.freighterSignTransaction = async function (xdr, network) {
        try {
            const api = getFreighterApi();
            if (!api) return '';

            const networkPassphrase = network === 'TESTNET'
                ? 'Test SDF Network ; September 2015'
                : 'Public Global Stellar Network ; September 2015';

            if (typeof api.signTransaction === 'function') {
                const result = await api.signTransaction(xdr, { networkPassphrase });
                return typeof result === 'string' ? result : (result && result.signedTxXdr ? result.signedTxXdr : '');
            }
            return '';
        } catch (error) {
            console.error('[Wallet] Sign error:', error);
            return '';
        }
    };

    console.log('[Wallet] 🔗 Freighter bridge loaded');
})();
