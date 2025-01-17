<script lang="ts">
	import { Modal, ProgressRing, type ToastContext } from '@skeletonlabs/skeleton-svelte';
	import { PollState, type Poll, type Upgrade } from '../../routes/+page.svelte';
	import { pollType } from '$lib';
	import { getContext } from 'svelte';

	interface Props {
		poll: Poll | null;
		upgrade: Upgrade;
		disabledClass: string;
		connected: boolean;
	}

	const toast: ToastContext = getContext('toast');

	let { poll, upgrade, disabledClass }: Props = $props();
	let openState = $state(false);
	let modalOpenedOnce = $state(false);
	let pollFinished = $state(false);

	$effect(() => {
		if (!modalOpenedOnce && poll && $pollType == '') {
			modalOpenedOnce = true;
			openState = true;
		}
	});

	$effect(() => {
		if (poll && poll.time_remaining == 0) pollFinished = true;
	});
	$effect(() => {
		if (pollFinished && upgrade.last_upgrade != PollState.Pending) setTimeout(createPollState, 0);
	});

	function modalClose() {
		openState = false;
	}

	function select(incPollType: string) {
		if (incPollType) $pollType = incPollType;
	}

	function createPollState() {
		pollFinished = false;
		toast.create({
			title: 'Poll Results',
			description: `Voted for ${upgrade.last_upgrade}!`,
			duration: 5_000
		});
	}
</script>

<Modal
	bind:open={openState}
	triggerBase="{disabledClass} btn preset-tonal-secondary border-[1px] border-secondary-500 shadow-sm shadow-secondary-500"
	contentBase="card preset-tonal-secondary border-[1px] border-secondary-500 p-4 space-y-4 shadow-xl max-w-screen-sm"
	backdropClasses="backdrop-blur-sm"
>
	{#snippet trigger()}
		{#if poll}
			<p>Upgrade Poll</p>
			<ProgressRing
				value={poll.time_remaining}
				max={1200}
				size="size-6"
				strokeWidth="4px"
				meterStroke="stroke-secondary-500"
				trackStroke="stroke-surface-950 dark:stroke-surface-100"
				strokeLinecap="round"
			/>
		{:else}
			<p>Upgrades</p>
		{/if}
	{/snippet}
	{#snippet content()}
		{#if poll}
			<header class="flex items-center justify-between">
				<h2 class="h2">Upgrade Poll x {poll.amplification}</h2>
				<ProgressRing
					value={poll.time_remaining}
					max={1200}
					size="size-16"
					strokeWidth="8px"
					meterStroke="stroke-secondary-500"
					trackStroke="stroke-surface-950 dark:stroke-surface-100"
					strokeLinecap="round"
				>
					<p class="text-sm">{Math.floor(poll.time_remaining / 4)}s</p>
				</ProgressRing>
			</header>
			<article>
				<p class="opacity-80">
					You'll now be able to vote for an upgrade, either for the <kbd class="kbd">Base</kbd>
					or <kbd class="kbd">Exponent</kbd> of the calculation!
					<br />
					Current upgrades: {upgrade.base} * <kbd class="kbd">Base</kbd>, {upgrade.exponent} *
					<kbd class="kbd">Exponent</kbd>
					<br />
					Current upgrade level: {upgrade.level}
				</p>
			</article>
			<footer class="flex justify-end gap-4">
				<button type="button" class="btn preset-tonal" onclick={modalClose}>Close</button>
				<button
					type="button"
					class="btn {$pollType == 'base' ? 'preset-filled' : 'preset-tonal-surface'}"
					onclick={() => select('base')}>Base {poll.meter.base}</button
				>
				<button
					type="button"
					class="btn {$pollType == 'exponent' ? 'preset-filled' : 'preset-tonal-surface'}"
					onclick={() => select('exponent')}>Exponent {poll.meter.exponent}</button
				>
			</footer>
		{:else}
			<header class="flex items-center justify-between">
				<h2 class="h2">Upgrades</h2>
			</header>
			<article>
				<p class="opacity-80">
					{#if upgrade.last_upgrade == PollState.Base}
						You've voted as your last upgrade to increase the <kbd class="kbd">Base</kbd> multiplying
						factor!
					{:else if upgrade.last_upgrade == PollState.Exponent}
						You've voted as your last upgrade to increase the <kbd class="kbd">Exponent</kbd> additive
						factor!
					{:else}
						No polls so far!
					{/if}
					<br />
					Current upgrades: {upgrade.base} * <kbd class="kbd">Base</kbd>, {upgrade.exponent} *
					<kbd class="kbd">Exponent</kbd>
					<br />
					Current upgrade level: {upgrade.level}
				</p>
			</article>
			<footer class="flex justify-end gap-4">
				<button type="button" class="btn preset-tonal" onclick={modalClose}>Close</button>
			</footer>
		{/if}
	{/snippet}
</Modal>
