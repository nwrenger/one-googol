<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';

	import IconMoon from 'lucide-svelte/icons/moon';
	import IconSun from 'lucide-svelte/icons/sun';
	import { onMount } from 'svelte';

	let mode = $state(false);
	onMount(() => {
		mode = localStorage.getItem('theme') === 'light';
	});

	function handleModeChange() {
		localStorage.setItem('theme', mode ? 'dark' : 'light');
	}

	$effect(() => {
		document.documentElement.classList.toggle('dark', !mode);
	});
</script>

<Switch
	name="mode"
	compact
	controlActive="bg-surface-none"
	controlInactive="bg-surface-none"
	controlPadding="p-1"
	bind:checked={mode}
	onCheckedChange={handleModeChange}
>
	{#snippet inactiveChild()}<IconMoon size="20" />{/snippet}
	{#snippet activeChild()}<IconSun size="20" />{/snippet}
</Switch>
