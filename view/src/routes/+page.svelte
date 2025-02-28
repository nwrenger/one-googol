<script lang="ts" module>
	export interface Counter {
		count: Count;
		poll: Poll | null;
		upgrade: Upgrade;
		type: CounterKind;
	}

	interface Count {
		value: string;
		meter: CountMeter;
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

	enum CounterKind {
		Auto = 'Auto',
		CookieClicker = 'CookieClicker'
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
	import { Segment, type ToastContext } from '@skeletonlabs/skeleton-svelte';
	import Confetti from 'svelte-confetti';
	import { increaseType, pollType } from '$lib';
	import { getContext, onDestroy } from 'svelte';
	import ClientCounter from '$lib/components/ClientCounter.svelte';
	import ModalUpgrades from '$lib/components/ModalUpgrades.svelte';

	const GOOGOL = (10n ** 100n).toString();
	const GOOGOL_LENGTH = 101;
	const toast: ToastContext = getContext('toast');

	let counter: Counter = $state({
		count: { value: '0', meter: { increment: 0, decrement: 0, pending: 0 } },
		poll: null,
		upgrade: { level: 0, last_upgrade: PollState.Pending, base: 1, exponent: 0 },
		type: CounterKind.Auto
	});
	let counter_splitted = $derived(counter.count.value.padStart(GOOGOL_LENGTH, '0').split(''));
	let socket = connect();
	let connected: boolean = $state(false);
	let interval: number | undefined = undefined;
	let googol_reached = $derived(counter.count.value === GOOGOL);
	let disabled = $derived(googol_reached || !connected);
	let disabledClass = $derived(disabled ? 'opacity-40 pointer-events-none' : '');

	$effect(() => {
		if (!connected) reconnect();
	});
	$effect(() => onIncreaseType($increaseType));
	$effect(() => onIncreaseType($pollType));

	onDestroy(() => {
		socket?.close();
		clearInterval(interval);
		createClosed();
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
		toast.create({ title: 'WebSocket', description: 'Connecting...', duration: 2_500 });
	}

	function createConnected() {
		toast.create({
			title: 'WebSocket',
			description: 'Connected!',
			type: 'success',
			duration: 2_500
		});
	}

	function createClosed() {
		toast.create({
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
				highlighted={(i - 1) % 10 == 0 && counter.count.value.length - 1 < Math.abs(100 - i)}
				{disabled}
			/>
		{/each}
	</div>

	<div class="flex items-center space-x-2 sm:space-x-3">
		<ClientCounter
			background="{disabledClass} border-primary-500 shadow-primary-500 preset-tonal-primary"
			increase={counter.count.meter.increment}
			{counter}
		/>

		<Segment
			name="increaseType"
			background="{disabledClass} preset-outlined-surface-950-50 preset-tonal-surface shadow-xs shadow-surface-950 dark:shadow-surface-50"
			value={$increaseType}
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
			background="{disabledClass} border-tertiary-500 shadow-tertiary-500 preset-tonal-tertiary"
			increase={-counter.count.meter.decrement}
			{counter}
		/>
	</div>

	<ModalUpgrades poll={counter.poll} upgrade={counter.upgrade} {disabledClass} {connected} />
</div>
