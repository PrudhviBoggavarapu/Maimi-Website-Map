<script lang="ts">
	import {
		Root as HoverCardRoot,
		Trigger as HoverCardTrigger,
		Content as HoverCardContent
	} from '$lib/components/ui/hover-card';
	import { Button } from '$lib/components/ui/button';
	// import CalendarDays here if you're using it, or remove the usage below
</script>

<HoverCardRoot>
	<HoverCardTrigger>
		<Button
			class="hover:underline underline-offset-4 rounded-sm focus-visible:outline-2 focus-visible:outline-offset-8 focus-visible:outline-black"
			on:click
		>
			@sveltejs
		</Button>
	</HoverCardTrigger>

	<HoverCardContent class="w-80">
		<div class="flex justify-between space-x-4">
			<div class="space-y-1">
				<h4 class="text-sm font-semibold">@sveltejs</h4>
				<p class="text-sm">Cybernetically enhanced web apps.</p>
				<div class="flex items-center pt-2">
					<span class="text-xs text-muted-foreground"> Joined September 2022 </span>
				</div>
			</div>
		</div>
	</HoverCardContent>
</HoverCardRoot>
