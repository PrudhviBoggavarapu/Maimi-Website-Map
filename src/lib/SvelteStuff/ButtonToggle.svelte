<script>
	let isDarkMode = false;

	function toggleDarkMode() {
		isDarkMode = !isDarkMode;
		if (isDarkMode) {
			document.body.classList.add('dark');
		} else {
			document.body.classList.remove('dark');
		}
	}
</script>

<button
	class="px-4 py-2 bg-blue-500 text-white font-medium text-sm leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out"
	on:click={toggleDarkMode}
>
	{isDarkMode ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
</button>
