import {
    isDarkReaderEnabled,
    get_api_and_store,
    selectedMuseum,
    responseData,
    cleanData,
    targetDarkMode,
    dark_mode_handler,
    DarkModeState,
    dataLoaded
} from '$lib/shared/stores';
import { derived, get } from 'svelte/store';

let lib: typeof import('$lib/wasm-lib/pkg/wasm_lib');
export async function get_data_from_api() {
    lib = await import('$lib/wasm-lib/pkg/wasm_lib');
    await lib.default();

    await selectedMuseum.subscribe((storeValue) => {
        if (storeValue) {
            const url = `https://na.iiivega.com/api/search-result/drawer/format-groups/${storeValue.id
                }/locations?tab=Museum%20Pass`;
            let x = get_api_and_store(url);
            x.then((_item: any) => { });
        }
    });
}

export async function clean_data() {
    lib = await import('$lib/wasm-lib/pkg/wasm_lib');
    await lib.default();
    responseData.subscribe((value) => {
        let currentData = value;
        if (currentData !== null) {
            const cleaned_data = currentData.available.map((item) => {
                return lib.get_best_match(item.itemLocationLabel);
            });
            cleanData.set(cleaned_data);
            dataLoaded.set(true);

        }
    });
}
function checkDarkReaderMeta() {
    isDarkReaderEnabled.set('querySelector' in document && !!document.querySelector('meta[name=darkreader]'));
}

export async function detectdarkReader() {
    const observer = new MutationObserver((mutationsList, observer) => {
        mutationsList.find((mutation) => {
            if (mutation.type === 'childList' || mutation.type === 'attributes') {
                checkDarkReaderMeta();
                return true;
            }
            return false;
        });
    });
    observer.observe(document.documentElement, {
        childList: true,
        attributes: true,
        subtree: true
    });

    checkDarkReaderMeta();
}
export async function better_dark_mode_handling() {
    dark_mode_handler.subscribe((state) => {
        const existingLock = document.head.querySelector('meta[name="darkreader-lock"]');
        const bodyClassList = document.body.classList;

        switch (state) {
            case DarkModeState.DarkReaderEnabledLightMode:
                // Handle Dark Reader Enabled + Light Mode
                if (!existingLock) {
                    const lock = document.createElement('meta');
                    lock.name = 'darkreader-lock';
                    document.head.appendChild(lock);
                }

                bodyClassList.remove('dark');
                break;
            case DarkModeState.DarkReaderEnabledDarkMode:
                // Handle Dark Reader Enabled + Dark Mode
                if (existingLock)
                    document.head.removeChild(existingLock);
                bodyClassList.add('dark');
                break;
            case DarkModeState.DarkReaderDisabledLightMode:
                // Handle Dark Reader Disabled + Light Mode
                bodyClassList.remove('dark');
                break;

            case DarkModeState.DarkReaderDisabledDarkMode:
                // Handle Dark Reader Disabled + Dark Mode
                bodyClassList.add('dark');
                break;
        }
    })


}


function onPageLoad() {
    console.log('Page is fully loaded');
}


export async function prelude_data() {
    await get_data_from_api();
    window.addEventListener('load', onPageLoad());
    await detectdarkReader();
    await clean_data();
    await better_dark_mode_handling();
}
