import { prove as tlsnProve } from "tlsn-js";
import browser from "webextension-polyfill";
import { useProofContext } from "./useProofContext";
import { useCallback, useEffect, useState } from "react";
export const useTlsnProover = () => {

	const { proofUrl } = useProofContext();
	const [proof, setProof] = useState<any>();
	const [isProoving, setIsProoving] = useState(false);
	const [ hasDataForProof, setHasDataForProof] = useState(false);
	const [cookies, setCookies] = useState<browser.Cookies.Cookie[]>([]);
	const [headers, setHeaders] = useState<browser.WebRequest.HttpHeadersItemType[]>([]);

	useEffect(() => {
		setHasDataForProof(cookies.length > 0 && headers.length > 0);
	}, [cookies, headers]);
	useEffect(() => {

		browser.webRequest.onResponseStarted.addListener(
			async (details) => {
				if (details.url.includes(proofUrl)) {
					const cookies = await browser.cookies.getAll({ url: details.url });
					setCookies(cookies);
				}
			},
			{ urls: ["<all_urls>"] },
		);

		browser.webRequest.onBeforeSendHeaders.addListener(
			(details) => {
				if (details.url.includes(proofUrl)) {
					const headers : browser.WebRequest.HttpHeadersItemType[] = []; 
					details.requestHeaders?.forEach((header) => {
						headers.push(header);
					});
					setHeaders(headers);
				}
			},
			{ urls: ["<all_urls>"] },
			["requestHeaders"]
		);
	}, []);

	const prove = useCallback(async () => {
		setIsProoving(true);
		const tlsnProof = await tlsnProve(proofUrl, {
			notaryUrl: import.meta.env.VITE_NOTARY_URL,
			websocketProxyUrl: import.meta.env.VITE_WEBSOCKET_PROXY_URL,
			headers : headers.reduce((acc, header) => {
				const headerName = header.name.toLowerCase();
				if ( header.value) {
					acc[headerName] = header.value;
				}
				return acc; 
			}, {} as Record<string, string>),

		})
		setProof(tlsnProof);
		setIsProoving(false);


	}, [cookies, headers]);
	return {
		prove,
		proof,
		isProoving,
		hasDataForProof,
	}
};

