<script lang="ts">
	import DetectDarkReader from '$lib/SvelteStuff/DetectDarkReader.svelte';
	import { isDarkReaderEnabled } from '$lib/shared/stores';
</script>

<!-- The UI part of your component -->
<div>
	Dark Reader is {$isDarkReaderEnabled ? 'enabled' : 'disabled'}.
	<DetectDarkReader />
</div>
