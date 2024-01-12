<script>
	import { responseData, loading, error } from '../../shared/stores';

	$: if ($loading) {
	} else if ($error) {
		console.error('Error:', $error);
	} else {
	}
</script>

<main class="font-sans p-5">
	<h1 class="text-2xl font-bold mb-3">Data Fetch Example</h1>

	{#if $loading}
		<p class="text-gray-500">Loading...</p>
	{:else if $error}
		<p class="text-red-500">Error: {$error}</p>
	{:else}
		<pre class="bg-gray-100 p-3 border border-gray-300 rounded-md overflow-auto">{JSON.stringify(
				$responseData,
				null,
				2
			)}</pre>
	{/if}
</main>
