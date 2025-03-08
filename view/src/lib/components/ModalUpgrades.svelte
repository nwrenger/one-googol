<script lang="ts">
	import { Modal, ProgressRing, type ToastContext } from '@skeletonlabs/skeleton-svelte';
	import { PollState, type Counter, type Poll, type Upgrade } from '../../routes/+page.svelte';
	import { pollType } from '$lib';
	import { getContext } from 'svelte';

	interface Props {
		counter: Counter;
		disabledClass: string;
		connected: boolean;
	}

	const toast: ToastContext = getContext('toast');

	let { counter, disabledClass }: Props = $props();
	let openState = $state(false);
	let modalOpenedOnce = $state(false);
	let pollFinished = $state(false);

	$effect(() => {
		if (!modalOpenedOnce && counter.poll && $pollType == '') {
			modalOpenedOnce = true;
			openState = true;
		}
	});

	$effect(() => {
		if (counter.poll && counter.poll.time_remaining == 0) pollFinished = true;
	});
	$effect(() => {
		if (pollFinished && counter.upgrade.last_upgrade != PollState.Pending)
			setTimeout(createPollState, 0);
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
			description: `Voted for ${counter.upgrade.last_upgrade}!`,
			duration: 2_500
		});
	}
</script>

<Modal
	open={openState}
	onOpenChange={(e) => (openState = e.open)}
	triggerBase="{disabledClass} btn preset-tonal-secondary border-[1px] border-secondary-500 shadow-xs shadow-secondary-500"
	contentBase="card preset-tonal-secondary border-[1px] border-secondary-500 p-4 space-y-4 shadow-xl max-w-(--breakpoint-sm)"
	backdropClasses="backdrop-blur-xs"
>
	{#snippet trigger()}
		{#if counter.poll}
			<p>Upgrade Poll</p>
			<ProgressRing
				value={counter.poll.time_remaining}
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
		{#if counter.poll}
			<header class="flex items-center justify-between">
				<h2 class="h2">Upgrade Poll x {counter.poll.amplification}</h2>
				<ProgressRing
					value={counter.poll.time_remaining}
					max={1200}
					size="size-16"
					strokeWidth="8px"
					meterStroke="stroke-secondary-500"
					trackStroke="stroke-surface-950 dark:stroke-surface-100"
					strokeLinecap="round"
				>
					<p class="text-sm">{Math.floor(counter.poll.time_remaining / 4)}s</p>
				</ProgressRing>
			</header>
			<article class="space-y-4 opacity-80">
				<p>
					You'll now be able to vote for an upgrade, either for the <kbd class="kbd">Base</kbd>
					or <kbd class="kbd">Exponent</kbd> of the calculation!
				</p>
				<ul class="list-inside list-disc space-y-2">
					<li>Upgrade Level: {counter.upgrade.level}</li>
					<li>
						Upgrades: {counter.upgrade.base} * <kbd class="kbd">Base</kbd>, {counter.upgrade
							.exponent} *
						<kbd class="kbd">Exponent</kbd>
					</li>
					<li>
						Formula: (player_count/upgrade_level * {counter.upgrade.base})<sup
							>√{counter.count.value.length} + {counter.upgrade.exponent}</sup
						>
					</li>
				</ul>
			</article>
			<footer class="flex justify-end gap-4">
				<button type="button" class="btn preset-tonal" onclick={modalClose}>Close</button>
				<button
					type="button"
					class="btn {$pollType == 'base' ? 'preset-filled' : 'preset-tonal-surface'}"
					onclick={() => select('base')}>Base {counter.poll.meter.base}</button
				>
				<button
					type="button"
					class="btn {$pollType == 'exponent' ? 'preset-filled' : 'preset-tonal-surface'}"
					onclick={() => select('exponent')}>Exponent {counter.poll.meter.exponent}</button
				>
			</footer>
		{:else}
			<header class="flex items-center justify-between">
				<h2 class="h2">Upgrades</h2>
			</header>
			<article class="space-y-4 opacity-80">
				<p>
					{#if counter.upgrade.last_upgrade == PollState.Base}
						You've voted as your last upgrade to increase the <kbd class="kbd">Base</kbd> multiplying
						factor!
					{:else if counter.upgrade.last_upgrade == PollState.Exponent}
						You've voted as your last upgrade to increase the <kbd class="kbd">Exponent</kbd> additive
						factor!
					{:else}
						No polls so far!
					{/if}
				</p>
				<ul class="list-inside list-disc space-y-2">
					<li>Upgrade Level: {counter.upgrade.level}</li>
					<li>
						Upgrades: {counter.upgrade.base} * <kbd class="kbd">Base</kbd>, {counter.upgrade
							.exponent} *
						<kbd class="kbd">Exponent</kbd>
					</li>
					<li>
						Formula: (player_count/upgrade_level * {counter.upgrade.base})<sup
							>√{counter.count.value.length} + {counter.upgrade.exponent}</sup
						>
					</li>
				</ul>
			</article>
			<footer class="flex justify-end gap-4">
				<button type="button" class="btn preset-tonal" onclick={modalClose}>Close</button>
			</footer>
		{/if}
	{/snippet}
</Modal>
