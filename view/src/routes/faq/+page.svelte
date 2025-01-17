<svelte:head>
	<title>FAQ - One Googol</title>
	<meta name="description" content="The FAQ of the One Googol Project!" />
</svelte:head>

<div class="mx-auto max-w-3xl space-y-8 pb-14 pt-10">
	<a href="/" class="anchor">← Back</a>

	<section class="space-y-6">
		<h3 class="h3">Welcome to One Googol!</h3>
		<p>
			One Googol is a collaborative project where you, the participants, work together to reach a
			number with 100 zeros, known as a googol.
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">How Does It Work?</h4>
		<p>
			On the <a href="/" class="anchor">main page</a> you'll find the counter, an
			<span class="text-primary-500">increment</span>
			and <span class="text-tertiary-500">decrement</span> selector, on each side of this selector
			are the counts of other clients and under that an
			<span class="text-secondary-500">Upgrades/Upgrade Poll</span> Button which opens a modal with upgrade
			info. The counter and poll counter/results will be shown to you in real-time (refreshed every 250ms)
			by utilizing a WebSocket connection to the server.
		</p>
		<p>
			The poll time, consisting of 5 Minutes, will only countdown if any option is at least voted
			once. Closing the counter view will close the WebSocket connection and, therefore, also remove
			your vote and counter selection!
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">How does the scaling work?</h4>
		<p>
			Since a googol is an enormous number, I've implemented automatic scaling. It works by raising
			the number of players to the power of the square root of the total counter length, with
			additional factors that players can vote on when they reach the <span
				class="text-secondary-500">violet-colored number cards</span
			>. Concretely:
		</p>
		<pre class="pre">step = player_count * base_factor<sup>√(counter_length + exponent_factor)</sup
			></pre>
		<p>
			This <strong>step</strong> value is added to the counter every 250 ms. When you hover over the
			player count cards, you can see the currently calculated step.
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">What Happens When One Googol Is Reached?</h4>
		<p>
			Then you've reached a number where it would take 3,1709791984×10<sup>92</sup> Years when every
			second its incremented by one. A number which is that big that it's over the Atom count in the
			visible universe by 10<sup>20</sup>. A number which Google inherited their name from. A number
			that is impossible to reach under normal circumstances! Isn't that enough?
		</p>
	</section>
</div>
