<script>
	import { goto } from '$app/navigation';
	import GiveMeMoney from '$lib/SvelteStuff/GiveMeMoney.svelte';
</script>

<div class="mx-auto p-4 max-w-7xl">
	<!-- Adjusted max-width -->
	<h1 class="text-2xl font-bold text-center mb-6">
		Welcome to the Free Museum Pass Program from the Miami-Dade Public Library System
	</h1>

	<div class="flex flex-col lg:flex-row">
		<!-- 'lg:flex-row' for larger screens -->
		<div class="lg:w-1/2 mb-4">
			<!-- Adjusted width for larger screens -->
			<p class="text-lg">
				Here you can find a pass for many of the museums in the Miami-Dade area. The pass grants
				free entry into the museums. You must physically go to the library to pick up a pass.
			</p>

			<p class="text-lg mt-4">
				Start by picking a museum and clicking the loading button. It will display the available
				passes on a map. This website requests your location to show a pin where you are.
			</p>

			<script src="https://liberapay.com/CodingPabs/widgets/button.js"></script>
			<noscript
				><a href="https://liberapay.com/CodingPabs/donate"
					><img
						alt="Donate using Liberapay"
						src="https://liberapay.com/assets/widgets/donate.svg"
					/></a
				></noscript
			>
			<br />
			<br />
		</div>

		<div class="lg:w-1/2 lg:pl-4 overflow-auto">
			<h2 class="text-xl font-semibold mb-3">How the Pass Works</h2>
			<ul class="list-disc list-inside mb-4 text-left">
				<li>Pass is good for up to a family of four.</li>
				<li>Pass is good for one visit.</li>
				<li>Pass is not renewable or available for reserve, i.e., no holding passes.</li>
				<li>
					Pass is valid for seven days from the time of check-out and will remain on the customer's
					record for the duration of the seven-day checkout.
				</li>
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
			<br /><br /><br />
		</div>
	</div>

	<div class="flex justify-center items-center flex-col">
		<button
			class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded transition duration-300 ease-in-out transform hover:-translate-y-1 hover:shadow-lg"
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
