<script lang="ts">
	import { ArrowBigDown, ArrowBigUp, CloudAlert } from 'lucide-svelte';
	import DigitScroller from '$lib/components/DigitScroller.svelte';
	import { Segment, Tooltip } from '@skeletonlabs/skeleton-svelte';
	import Confetti from 'svelte-confetti';
	import { onNavigate } from '$app/navigation';
	import { increaseType } from '$lib';

	const GOOGOL = (10n ** 100n).toString();
	const GOOGOL_LENGTH = 101;

	let counter = $state('0');
	let counter_splitted = $derived(counter.padStart(GOOGOL_LENGTH, '0').split(''));
	let clients = $state(['0', '0']);
	let socket = connect();
	let connected = $state(false);
	let tooltipOpen = $state(false);

	$effect(() => onIncreaseType($increaseType));
	onNavigate(() => socket.close());

	function connect(): WebSocket {
		let new_socket = new WebSocket(`/ws`);

		new_socket.onopen = () => {
			connected = true;
		};

		new_socket.onmessage = (event) => {
			[counter, clients[0], clients[1]] = (event.data as string).split(',');
			connected = true;
		};

		new_socket.onclose = () => {
			connected = false;
		};

		return new_socket;
	}

	function onIncreaseType(increaseType: string) {
		if (connected && increaseType != '') {
			socket.send(increaseType);
		}
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
				class="border-surface-950 shadow-surface-950 preset-tonal-surface dark:border-surface-50 dark:shadow-surface-50 rounded-lg border-[1px] p-8 text-center shadow-sm"
			>
				<h2 class="h3 text-success-500 md:h2 md:text-success-500 mb-4">
					🎉 Congratu&shy;lations! 🎉
				</h2>
				<h5 class="h6 md:h5 font-normal">You've reached One Googol!</h5>
			</div>
		</div>
	{/if}

	<div class="mx-auto flex w-full max-w-screen-xl flex-wrap justify-center gap-2">
		{#each counter_splitted as digit}
			<DigitScroller {digit} finished={counter === GOOGOL} />
		{/each}
	</div>

	<div class="flex items-center space-x-2 sm:space-x-3">
		<div
			class="card {counter === GOOGOL
				? 'opacity-40'
				: ''} border-primary-500 shadow-primary-500 preset-tonal-primary relative flex h-10 w-fit min-w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-sm transition-transform duration-300"
		>
			{clients[0]}
		</div>

		<Segment
			name="increaseType"
			background="preset-outlined-surface-950-50 preset-tonal-surface shadow-sm shadow-surface-950 dark:shadow-surface-50"
			bind:value={$increaseType}
		>
			<Segment.Item disabled={!(connected && counter != GOOGOL)} value="increment">
				<ArrowBigUp class="text-primary-500" />
			</Segment.Item>
			<Segment.Item
				disabled={!(connected && counter != '0' && counter != GOOGOL)}
				value="decrement"
			>
				<ArrowBigDown class="text-tertiary-500" />
			</Segment.Item>
		</Segment>

		<div
			class="card {counter === GOOGOL
				? 'opacity-40'
				: ''} border-tertiary-500 shadow-tertiary-500 preset-tonal-tertiary relative flex h-10 w-fit min-w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-sm transition-transform duration-300"
		>
			{clients[1]}
		</div>

		{#if !connected}
			<Tooltip
				bind:open={tooltipOpen}
				positioning={{ placement: 'top' }}
				base="flex items-center"
				contentBase="card preset-filled-error-500 p-4"
				openDelay={0}
				closeOnClick={false}
				closeOnPointerDown={false}
				onclick={() => {
					if (tooltipOpen) {
						socket = connect();
						tooltipOpen = false;
					} else {
						tooltipOpen = true;
					}
				}}
			>
				{#snippet trigger()}
					<CloudAlert class="text-error-500" />
				{/snippet}
				{#snippet content()}
					<p class="text-center text-sm">
						No WebSocket connection established!
						<br />
						Click again to reconnect!
					</p>
				{/snippet}
			</Tooltip>
		{/if}
	</div>
</div>
