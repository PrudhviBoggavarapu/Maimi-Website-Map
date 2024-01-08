import type {Location}
from '$lib/wasm-lib/pkg/wasm_lib';
import {get, writable} from 'svelte/store';
import type {Writable}
from 'svelte/store';
export const cleanData: Writable < Location[] | null > = writable(null);
export const responseData: Writable < StoreData | null > = writable(null);
export const loading = writable(false);
export const error = writable(null);
export const dataLoaded = writable(false);
export const isMapLoading = writable(true);
export const storeForOutputOfNotificaionData = writable(true);

export function urlBase64ToUint8Array(base64String : string | any[]) {
    const padding = '='.repeat((4 - (base64String.length % 4)) % 4);
    const base64 = (base64String + padding).replace(/\-/g, '+').replace(/_/g, '/');

    const rawData = window.atob(base64);
    const outputArray = new Uint8Array(rawData.length);

    for (let i = 0; i < rawData.length; ++ i) {
        outputArray[i] = rawData.charCodeAt(i);
    }
    return outputArray;
}


export interface LocationItem {
    code: string;
    label: string;
    isPatronHome: boolean;
    itemLocationLabel: string;
    callNumber?: string;
}

export interface StoreData {
    available: LocationItem[];
    unavailable: LocationItem[];
}

export interface Museum {
    id: string;
    title: string;
    url: string;
}
export function museumToJSON(museum : Museum): Record < string,
any > {
    return {id: museum.id, title: museum.title, url: museum.url};
}


export const museums: Museum[] = [
    {
        id: "8c8127e9-7ff5-4bb1-8127-e97ff5abb1ef",
        title: "Black Police Precinct & Courthouse Museum",
        url: "https://historicalblackprecinct.org/"
    },
    {
        id: "abe1f830-b9f1-4f94-a1f8-30b9f13f948a",
        title: "HistoryMiami",
        url: "http://www.historymiami.org"
    },
    {
        id: "090a7a3c-6ba4-458a-8a7a-3c6ba4558a72",
        title: "Zoo Miami",
        url: "http://www.miamimetrozoo.com/"
    },
    {
        id: "3feb985f-4e4b-4d06-ab98-5f4e4bcd0634",
        title: "Museum of Graffiti",
        url: "http://museumofgraffiti.com/"
    }, {
        id: "65f3092e-a82b-466c-b309-2ea82b266c9f",
        title: "The Bass",
        url: "http://www.thebass.org/"
    }, {
        id: "7a6a43b5-37d2-4e9e-aa43-b537d2ae9e1d",
        title: "Perez Art Museum Miami",
        url: "http://www.pamm.org"
    }, {
        id: "83332190-cbea-4de8-b321-90cbeafde82d",
        title: "The Fruit and Spice Park",
        url: "http://redlandfruitandspice.com"
    }, {
        id: "0603917f-9ddf-4c0b-8391-7f9ddf5c0b62",
        title: "The Coral Gables Museum",
        url: "http://coralgablesmuseum.org/"
    }, {
        id: "22627b49-3888-4d4b-a27b-4938889d4b0b",
        title: "Phillip and Patricia Frost Museum of Science",
        url: "http://www.frostscience.org/"
    }, {
        id: "90fcd072-d05f-4575-bcd0-72d05f357563",
        title: "Miami Children's Museum",
        url: "http://www.miamichildrensmuseum.org/"
    }
];export const selectedMuseum: Writable < Museum > = writable(museums[0]);export const isDarkReaderEnabled = writable(false);export async function get_api_and_store(url : RequestInfo | URL) {
if (get(dataLoaded)) {
    return;
}


loading.set(true);
error.set(null);

const headers = {
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0',
    Accept: 'application/json, text/plain, */*',
    'Accept-Language': 'en-US,en;q=0.5',
    'Accept-Encoding': 'gzip, deflate, br',
    'api-version': '1',
    'iii-customer-domain': 'mdpls.na.iiivega.com',
    'iii-host-domain': 'mdpls.na.iiivega.com',
    Origin: 'https://mdpls.na.iiivega.com',
    Connection: 'keep-alive',
    Referer: 'https://mdpls.na.iiivega.com/',
    'Sec-Fetch-Dest': 'empty',
    'Sec-Fetch-Mode': 'cors',
    'Sec-Fetch-Site': 'same-site',
    Pragma: 'no-cache',
    'Cache-Control': 'no-cache',
    TE: 'trailers'
};

const response = await fetch(url, {headers});
if (! response.ok) {
    throw new Error(`HTTP error! status: ${
        response.status
    }`);
}
const data: StoreData = await response.json();
responseData.set(data);
dataLoaded.set(true);
return data;};export function createGoogleMapsURL(address : string) { // Encode the address
var encodedAddress = encodeURIComponent(address);

// Create the Google Maps URL
var googleMapsURL = "https://www.google.com/maps/search/?api=1&query=" + encodedAddress;

return googleMapsURL;};export function getCurrentLocation() { // Check if Geolocation is supported
if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(showPosition, showError);
} else {
    console.log("Geolocation is not supported by this browser.");
}}export function showPosition(position : any) {
console.log("Latitude: " + position.coords.latitude + "\nLongitude: " + position.coords.longitude);}export function showError(error : any) {
switch (error.code) {
    case error.PERMISSION_DENIED:
        console.log("User denied the request for Geolocation.");
        break;
    case error.POSITION_UNAVAILABLE:
        console.log("Location information is unavailable.");
        break;
    case error.TIMEOUT:
        console.log("The request to get user location timed out.");
        break;
    case error.UNKNOWN_ERROR:
        console.log("An unknown error occurred.");
        break;
}}
