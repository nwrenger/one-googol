<script lang="ts" module>
	export interface Counter {
		count: Count;
		poll: Poll | null;
		upgrade: Upgrade;
	}

	export interface Count {
		value: string;
		meter: CountMeter;
		accumulated_actions: number;
	}

	export interface Poll {
		time_remaining: number;
		amplification: number;
		meter: PollMeter;
	}

	export interface Upgrade {
		level: number;
		last_upgrade: PollState;
		base: number;
		exponent: number;
	}

	interface CountMeter {
		increment: number;
		decrement: number;
		pending: number;
	}

	interface PollMeter {
		base: number;
		exponent: number;
		pending: number;
	}

	export enum PollState {
		Pending = 'Pending',
		Base = 'Base',
		Exponent = 'Exponent'
	}
</script>

<script lang="ts">
	import { ArrowBigDown, ArrowBigUp } from 'lucide-svelte';
	import DigitScroller from '$lib/components/DigitScroller.svelte';
	import { toaster } from './+layout.svelte';
	import { Segment } from '@skeletonlabs/skeleton-svelte';
	import Confetti from 'svelte-confetti';
	import { increaseType, pollType } from '$lib';
	import { onDestroy } from 'svelte';
	import ClientCounter from '$lib/components/ClientCounter.svelte';
	import ModalUpgrades from '$lib/components/ModalUpgrades.svelte';
	import ActionButton from '$lib/components/ActionButton.svelte';

	const GOOGOL = (10n ** 100n).toString();
	const GOOGOL_LENGTH = 101;

	let counter: Counter = $state({
		count: {
			value: '0',
			meter: { increment: 0, decrement: 0, pending: 0 },
			accumulated_actions: 0
		},
		poll: null,
		upgrade: { level: 0, last_upgrade: PollState.Pending, base: 1, exponent: 0 }
	});
	let counter_splitted = $derived(counter.count.value.padStart(GOOGOL_LENGTH, '0').split(''));
	let socket = connect();
	let connected: boolean = $state(false);
	let interval: number | undefined = undefined;
	let googol_reached = $derived(counter.count.value === GOOGOL);
	let disabled = $derived(googol_reached || !connected);

	$effect(() => {
		if (!connected) reconnect();
	});
	$effect(() => onIncreaseType($increaseType));
	$effect(() => onIncreaseType($pollType));

	onDestroy(() => {
		socket?.close();
		clearInterval(interval);
		// Arbitrary timeout needed for preserving state
		setTimeout(createClosed, 1);
	});

	function connect(): WebSocket | undefined {
		let new_socket = new WebSocket(`/ws`);

		new_socket.onopen = () => {
			clearInterval(interval);
			createConnected();
			connected = true;
		};

		new_socket.onmessage = (event) => {
			let data: Counter = JSON.parse(event.data);
			counter = data;
			connected = true;
		};

		new_socket.onclose = () => {
			connected = false;
		};

		return new_socket;
	}

	function reconnect() {
		// Because toast changes, I need to decouple from effect!
		setTimeout(createConnecting, 0);
		interval = setInterval(() => {
			createConnecting();
			socket = connect();
		}, 2_500);
	}

	function createConnecting() {
		toaster.create({
			title: 'WebSocket',
			description: 'Connecting...',
			duration: 2_500
		});
	}

	function createConnected() {
		toaster.create({
			title: 'WebSocket',
			description: 'Connected!',
			type: 'success',
			duration: 2_500
		});
	}

	function createClosed() {
		toaster.create({
			title: 'WebSocket',
			description: 'Connection Closed!',
			type: 'error',
			duration: 2_500
		});
	}

	function onIncreaseType(increaseType: string) {
		if (connected && increaseType != '') {
			socket?.send(increaseType);
		}
	}

	function onAction() {
		if (connected && $increaseType != '') {
			socket?.send('action');
		}
	}

	let actionBackground = $derived(
		$increaseType != ''
			? $increaseType == 'increment'
				? 'border-primary-500 shadow-primary-500 preset-tonal-primary'
				: 'border-tertiary-500 shadow-tertiary-500 preset-tonal-tertiary'
			: 'border-surface-950 shadow-surface-950 preset-tonal-surface dark:border-surface-50 dark:shadow-surface-50'
	);
</script>

<svelte:head>
	<title>One Googol</title>
	<meta name="description" content="The One Googol Project!" />
</svelte:head>

<div class="flex flex-col items-center justify-center space-y-6 p-4">
	{#if googol_reached}
		<div
			class="pointer-events-none fixed top-[-50px] left-0 flex h-[100vh] w-[100vw] justify-center overflow-hidden"
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
			<div class="absolute top-[40%] right-[10%]">
				<Confetti amount={200} infinite />
			</div>
			<div class="absolute top-[40%] right-[90%]">
				<Confetti amount={200} infinite />
			</div>
			<div
				class="border-surface-950 shadow-surface-950 preset-tonal-surface dark:border-surface-50 dark:shadow-surface-50 rounded-lg border-[1px] p-8 text-center shadow-xs"
			>
				<h2 class="h3 text-success-500 md:h2 md:text-success-500 mb-4">
					🎉 Congratu&shy;lations! 🎉
				</h2>
				<h5 class="h6 md:h5 font-normal">You've reached One Googol!</h5>
			</div>
		</div>
	{/if}

	<div class="mx-auto flex w-full max-w-(--breakpoint-xl) flex-wrap justify-center gap-2">
		{#each counter_splitted as digit, i}
			<DigitScroller
				{digit}
				highlighted={(i - 9) % 10 == 0 && counter.count.value.length - 1 < Math.abs(100 - i)}
				{disabled}
			/>
		{/each}
	</div>

	<div class="flex items-center space-x-2 sm:space-x-3">
		<ClientCounter
			background="border-primary-500 shadow-primary-500 preset-tonal-primary"
			increase={counter.count.meter.increment}
			{counter}
			{disabled}
		/>

		<Segment
			name="increaseType"
			background="preset-outlined-surface-950-50 preset-tonal-surface shadow-xs shadow-surface-950 dark:shadow-surface-50 disabled:pointer-events-none disabled:opacity-40"
			value={$increaseType}
			{disabled}
			onValueChange={(e) => ($increaseType = e.value || '')}
		>
			<Segment.Item {disabled} value="increment">
				<ArrowBigUp class="text-primary-500" />
			</Segment.Item>
			<Segment.Item {disabled} value="decrement">
				<ArrowBigDown class="text-tertiary-500" />
			</Segment.Item>
		</Segment>

		<ClientCounter
			background="border-tertiary-500 shadow-tertiary-500 preset-tonal-tertiary"
			increase={-counter.count.meter.decrement}
			{counter}
			{disabled}
		/>
	</div>

	<ActionButton onclick={onAction} {counter} {disabled} background={actionBackground} />

	<ModalUpgrades {counter} {disabled} />
</div>
