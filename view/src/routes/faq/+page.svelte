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
			Because a googol is a massive number, I've implement automatic scaling. It works by
			calculating first a Step Level, where the number of digits are grouped into sets of three
			using this formula:
		</p>
		<pre class="pre">Step Level = floor((digits - 1) / 3)</pre>
		<p>And then this Step Level is used to determine the Step Size:</p>
		<pre class="pre">Step Size = 100<sup>Step Level</sup></pre>
		<p>
			On each increase and decrease the Step Size is used. It's also visible on the counter itself,
			by the inactive and grayed out digits.
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
