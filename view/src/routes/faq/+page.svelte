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
			and <span class="text-tertiary-500">decrement</span> selector and on each side of this selector
			are the counts of other clients. This will be shown to you in real-time (refreshed every 250ms)
			by utilizing a WebSocket connection to the server.
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">How does the scaling work?</h4>
		<p>
			Since a googol is an enormous number, I've implemented automatic scaling. It works by raising
			the number of players to the power of the square root of the total counter length:
		</p>
		<pre class="pre">step = player_count<sup>√counter_length</sup></pre>
		<p>This value is added to the counter every 250 ms.</p>
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
