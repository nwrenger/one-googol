<script lang="ts">
	import { ArrowBigDown, ArrowBigUp, Info, Loader, RefreshCcw } from 'lucide-svelte';
	import DigitScroller from '$lib/components/DigitScroller.svelte';
	import { Segment, type ToastContext } from '@skeletonlabs/skeleton-svelte';
	import Confetti from 'svelte-confetti';
	import { increaseType } from '$lib';
	import { getContext, onDestroy } from 'svelte';

	const GOOGOL = (10n ** 100n).toString();
	const GOOGOL_LENGTH = 101;
	const toast: ToastContext = getContext('toast');

	let counter = $state('0');
	let counter_splitted = $derived(counter.padStart(GOOGOL_LENGTH, '0').split(''));
	let clients = $state(['0', '0']);
	let socket = connect();
	let connected: boolean = $state(false);
	let interval: number | undefined = undefined;
	let googol_reached = $derived(counter === GOOGOL);
	let disabled = $derived(googol_reached || !connected);
	let disabledClass = $derived(disabled ? 'opacity-40' : '');

	$effect(() => {
		if (!connected) reconnect();
	});
	$effect(() => onIncreaseType($increaseType));
	onDestroy(() => {
		socket?.close();
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
			[counter, clients[0], clients[1]] = (event.data as string).split(',');
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
			socket = connect();
			createConnecting();
		}, 5_000);
	}

	function createConnecting() {
		toast.create({ title: 'WebSocket', description: 'Connecting...', duration: 5_000 });
	}

	function createConnected() {
		toast.create({
			title: 'WebSocket',
			description: 'Connected!',
			type: 'success',
			duration: 5_000
		});
	}

	function createClosed() {
		toast.create({
			title: 'WebSocket',
			description: 'Connection Closed!',
			type: 'error',
			duration: 5_000
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
			<DigitScroller {digit} {disabled} />
		{/each}
	</div>

	<div class="flex items-center space-x-2 sm:space-x-3">
		<div
			class="{disabledClass} card border-primary-500 shadow-primary-500 preset-tonal-primary relative flex h-10 w-fit min-w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-sm transition-transform duration-300"
		>
			{clients[0]}
		</div>

		<Segment
			name="increaseType"
			background="{disabledClass} preset-outlined-surface-950-50 preset-tonal-surface shadow-sm shadow-surface-950 dark:shadow-surface-50"
			bind:value={$increaseType}
		>
			<Segment.Item {disabled} value="increment">
				<ArrowBigUp class="text-primary-500" />
			</Segment.Item>
			<Segment.Item {disabled} value="decrement">
				<ArrowBigDown class="text-tertiary-500" />
			</Segment.Item>
		</Segment>

		<div
			class="{disabledClass} card border-tertiary-500 shadow-tertiary-500 preset-tonal-tertiary relative flex h-10 w-fit min-w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-sm transition-transform duration-300"
		>
			{clients[1]}
		</div>
	</div>
</div>
