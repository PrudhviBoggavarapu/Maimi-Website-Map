<script>
	import { goto } from '$app/navigation';
	import GiveMeMoney from '$lib/SvelteStuff/GiveMeMoney.svelte';
</script>

<div class="mx-auto p-4 max-w-7xl">
	<!-- Adjusted max-width -->
	<div>
		<h1 class="text-2xl font-bold text-center mb-2">Welcome to the Free Museum Pass Program</h1>
		<h2 class="text-1xl font-bold text-center mb-6">from the Miami-Dade Public Library System</h2>
	</div>

	<div class="flex flex-col lg:flex-row">
		<!-- 'lg:flex-row' for larger screens -->
		<div class="lg:w-1/2 mb-4">
			<!-- Adjusted width for larger screens -->
			<p class="text-lg">
				Here you can find a pass for many of the museums in the Miami-Dade area. The pass grants
				free entry into the museums. You must physically go to the library to check out a pass.
			</p>

			<p class="text-lg mt-4">
				Start by picking a museum from the dropdown, then click the load button. This will display
				the libraies that have that a pass available.
			</p>

			<p class="text-lg mt-4">
				This website requests your location to place a pin where you are. <strong>
					We do not collect your data.
				</strong>
			</p>

			<script src="jsFiles/button.js"></script>
			<noscript
				><a href="https://liberapay.com/CodingPabs/donate"
					><img alt="Donate using Liberapay" src="jsFiles/donateLibre.svg" /></a
				></noscript
			>
			<br />
			<br />
		</div>

		<div class="lg:w-7/12 lg:pl-4 overflow-auto">
			<h2 class="text-xl font-semibold mb-3">How the Pass Works</h2>
			<ul class="list-disc list-inside ml-5 mb-4 text-left">
				<li>Pass is good for up to a family of four.</li>
				<li>Pass is good for only one visit.</li>
				<li>Pass is not renewable or available for reserve.</li>
				<li>Pass is valid for seven days from the time of check-out</li>
				<li>Pass will remain on the customer's record for the duration of the checkout.</li>
				<li>Passes are available on a first-come, first-served basis.</li>
				<li>Only one pass may be checked out at a time.</li>
				<li>Limited number of passes per branch.</li>
				<li>Pass will not be replaced if lost or stolen.</li>
			</ul>
			<p class="italic mb-4">
				* Deering Estate family of four is two adults and two children ages 4-14.<br />
				* Fruit & Spice Park family of four is two adults and two children ages 6-11.<br />
				* Zoo Miami family of four is two adults and two children ages 3-12.
			</p>
			<a href="https://mdpls.org/museum-pass" class="underline">Official website</a>
		</div>
	</div>

	<div class="flex justify-center items-center flex-col">
		<button
			class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded transition duration-300 ease-in-out transform hover:-translate-y-1 hover:shadow-lg m-6"
			on:click={() => {
				localStorage.setItem('SawWelcome', 'true');
				goto('/');
			}}
			>I have read everything and i understand what this website is about
		</button>
		<br /><br /><br /><br />
		<GiveMeMoney />
	</div>
</div>
