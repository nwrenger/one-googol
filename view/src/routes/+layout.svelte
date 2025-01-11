<script lang="ts">
	import Lightswitch from '$lib/components/Lightswitch.svelte';
	import { ToastProvider } from '@skeletonlabs/skeleton-svelte';
	import { Dot } from 'lucide-svelte';
	import '../app.css';

	let { children } = $props();
</script>

<ToastProvider>
	<div class="mx-auto flex h-full max-w-7xl flex-col px-4 py-5">
		<header class="flex-shrink-0">
			<div class="flex h-12 w-full items-center justify-between">
				<div class="hidden sm:flex">
					<a href="/" title="One Googol">
						<h2 class="h2">10<sup>100</sup></h2>
					</a>
				</div>
				<div class="flex items-center space-x-2">
					<a href="/faq" class="anchor text-black dark:text-white">How to Play</a>
					<Dot />
					<a
						href="https://github.com/nwrenger/one-googol"
						target="_blank"
						rel="noopener noreferrer"
						class="anchor text-black dark:text-white"
					>
						Github
					</a>
					<Dot />
					<Lightswitch />
				</div>
			</div>
		</header>

		<main class="flex flex-grow items-center justify-center">
			{@render children()}
		</main>
	</div>
</ToastProvider>
