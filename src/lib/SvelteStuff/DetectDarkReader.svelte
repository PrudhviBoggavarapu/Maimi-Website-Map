<script>
	import { isDarkReaderEnabled } from '$lib/shared/stores';
	import { onMount, onDestroy } from 'svelte';

	// Function to check for the dark reader meta tag
	function checkDarkReaderMeta() {
		isDarkReaderEnabled.set(
			'querySelector' in document && !!document.querySelector('meta[name=darkreader]')
		);
	}

	onMount(() => {
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
	});
</script>
