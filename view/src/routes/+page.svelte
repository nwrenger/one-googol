<script lang="ts">
	import { ArrowBigDown, ArrowBigUp, CloudAlert } from 'lucide-svelte';
	import DigitScroller from '$lib/components/DigitScroller.svelte';
	import { ProgressRing, Tooltip } from '@skeletonlabs/skeleton-svelte';

	const GOOGOL = (10n ** 100n).toString();
	const GOOGOL_LENGTH = 101;

	let counter = $state('0');
	let counter_splitted = $derived(counter.padStart(GOOGOL_LENGTH, '0').split(''));
	let step = $derived(calculateStepLevel(counter));
	let socket = new WebSocket(`/ws`);
	let updatingIncr = $state(false);
	let updatingDecr = $state(false);
	let connected = $state(false);

	socket.onopen = () => {
		connected = true;
	};

	socket.onmessage = (event) => {
		counter = event.data;
		connected = true;
		updatingIncr = false;
		updatingDecr = false;
	};

	socket.onclose = () => {
		connected = false;
	};

	function increment() {
		if (connected && socket) {
			socket.send('increment');
			updatingIncr = true;
		}
	}

	function decrement() {
		if (connected && socket) {
			socket.send('decrement');
			updatingDecr = true;
		}
	}

	function calculateStepLevel(counterStr: string): number {
		const digits = counterStr.length;
		return Math.floor((digits - 1) / 3);
	}
</script>

<svelte:head>
	<title>One Googol</title>
	<meta name="description" content="The One Googol Project!" />
</svelte:head>

<div class="flex flex-col items-center justify-center space-y-6 p-4">
	<div class="mx-auto flex w-full max-w-screen-xl flex-wrap justify-center gap-2">
		{#each counter_splitted as digit, i}
			<DigitScroller {digit} finished={step * 2 > Math.abs(i - 100)} />
		{/each}
	</div>

	<div class="flex items-center space-x-2 sm:space-x-3">
		{#if !connected}
			<Tooltip
				positioning={{ placement: 'top' }}
				base="flex items-center"
				contentBase="card preset-filled-error-500 p-4"
				openDelay={200}
			>
				{#snippet trigger()}
					<CloudAlert class="text-error-500" />
				{/snippet}
				{#snippet content()}
					No WebSocket connection established!
				{/snippet}
			</Tooltip>
		{/if}

		<button
			disabled={!(connected && counter != GOOGOL)}
			title="Increment Counter"
			class="btn-icon shadow-primary-500 preset-filled-primary-500 shadow-sm"
			onclick={increment}
		>
			{#if updatingIncr}
				<ProgressRing
					value={null}
					size="size-4"
					strokeWidth="3px"
					meterStroke="stroke-surface-50"
					trackStroke="stroke-surface-950"
				/>
			{:else}
				<ArrowBigUp />
			{/if}
		</button>

		<button
			disabled={!(connected && counter != '0' && counter != GOOGOL)}
			title="Decrement Counter"
			class="btn-icon shadow-tertiary-500 preset-filled-tertiary-500 shadow-sm"
			onclick={decrement}
		>
			{#if updatingDecr}
				<ProgressRing
					value={null}
					size="size-4"
					strokeWidth="3px"
					meterStroke="stroke-surface-50"
					trackStroke="stroke-surface-950"
				/>
			{:else}
				<ArrowBigDown />
			{/if}
		</button>
	</div>
	<div>
		Step Level: <code class="code">{step}</code>
	</div>
</div>
