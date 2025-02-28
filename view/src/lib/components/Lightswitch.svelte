<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';

	import IconMoon from 'lucide-svelte/icons/moon';
	import IconSun from 'lucide-svelte/icons/sun';
	import { onMount } from 'svelte';

	let mode = $state(false);
	onMount(() => {
		mode = localStorage.getItem('theme') === 'light';
	});

	function handleModeChange(e: { checked: boolean }) {
		mode = e.checked;
		localStorage.setItem('theme', mode ? 'light' : 'dark');
	}

	$effect(() => {
		document.documentElement.classList.toggle('dark', !mode);
	});
</script>

<Switch
	checked={mode}
	onCheckedChange={handleModeChange}
	name="mode"
	compact
	controlActive="bg-surface-none"
	controlInactive="bg-surface-none"
	controlPadding="p-1"
>
	{#snippet inactiveChild()}<IconMoon size="20" />{/snippet}
	{#snippet activeChild()}<IconSun size="20" />{/snippet}
</Switch>
