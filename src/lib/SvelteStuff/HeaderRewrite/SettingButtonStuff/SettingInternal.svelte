<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
</script>

<form class="grid items-start gap-4">
	<div class="flex items-center space-x-2">
		<Switch id="Dark-Mode" />
		<Label for="Dark-Mode">Dark Mode</Label>
	</div>
</form>
