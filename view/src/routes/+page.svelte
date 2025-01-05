<script lang="ts">
	import { ArrowBigDown, ArrowBigUp, CloudAlert } from 'lucide-svelte';
	import DigitScroller from '$lib/components/DigitScroller.svelte';
	import { ProgressRing, Tooltip } from '@skeletonlabs/skeleton-svelte';
	import Confetti from 'svelte-confetti';

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
	{#if counter === GOOGOL}
		<div
			class="pointer-events-none fixed left-0 top-[-50px] flex h-[100vh] w-[100vw] justify-center overflow-hidden"
		>
			<Confetti
				x={[-5, 5]}
				y={[0, 0.1]}
				delay={[0, 2000]}
				infinite
				duration={5000}
				amount={200}
				fallDistance="100vh"
			/>
		</div>
		<div class="relative">
			<div class="absolute right-[10%] top-[40%]">
				<Confetti amount={200} infinite />
			</div>
			<div class="absolute right-[90%] top-[40%]">
				<Confetti amount={200} infinite />
			</div>
			<div
				class="rounded-lg border-[1px] border-surface-950 p-8 text-center shadow-sm shadow-surface-950 preset-tonal-surface dark:border-surface-50 dark:shadow-surface-50"
			>
				<h2 class="h3 mb-4 text-success-500 md:h2 md:text-success-500">
					🎉 Congratu&shy;lations! 🎉
				</h2>
				<h5 class="h6 font-normal md:h5">You've reached One Googol!</h5>
			</div>
		</div>
	{/if}

	<div class="mx-auto flex w-full max-w-screen-xl flex-wrap justify-center gap-2">
		{#each counter_splitted as digit, i}
			<DigitScroller {digit} finished={step * 2 > Math.abs(i - 100) || counter === GOOGOL} />
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
			class="btn-icon shadow-sm shadow-primary-500 preset-filled-primary-500"
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
			class="btn-icon shadow-sm shadow-tertiary-500 preset-filled-tertiary-500"
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
	<p class="transition-all duration-300 {counter === GOOGOL ? 'opacity-40' : ''}">
		Step Level: <code class="code">{step}</code>
	</p>
</div>
