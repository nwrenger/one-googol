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
			number with 100 zeros, known as one Googol.
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">How Does It Work?</h4>
		<p>
			The website utilizes WebSockets to maintain concurrent connections and provide real-time
			counter updates. If you want to begin to flood the site with counter updates, use the api:
		</p>
		<dl class="space-y-4">
			<div>
				<dt class="flex items-center space-x-3 font-bold">
					<span>GET /count</span> <code class="code">GET</code>
				</dt>
				<dd class="opacity-60">Retrieves the current value of the counter.</dd>
			</div>
			<div>
				<dt class="flex items-center space-x-3 font-bold">
					<span>POST /count/increment</span> <code class="code">POST</code>
				</dt>
				<dd class="opacity-60">
					Increments the counter by the current Step Size and returns the updated value.
				</dd>
			</div>
			<div>
				<dt class="flex items-center space-x-3 font-bold">
					<span>POST /count/decrement</span> <code class="code">POST</code>
				</dt>
				<dd class="opacity-60">
					Decrements the counter by the current Step Size and returns the updated value.
				</dd>
			</div>
		</dl>
		<p>
			Be aware that it's not the best Idea to overflood the server with too many requests. Although
			there aren't any software limits, there are hardware limits! Find the ideal spot!
		</p>
	</section>

	<section class="space-y-6">
		<h4 class="h4">How can the Step Size be determined?</h4>
		<p>The Step Size can be calculated by the Step Level:</p>
		<pre class="pre">stepSize = 100<sup>stepLevel</sup></pre>
	</section>

	<section class="space-y-6">
		<h4 class="h4">What are Step Levels?</h4>
		<p>
			A <code class="code">Step Level</code> dynamically adjusts the increment or decrement value based
			on the counter's magnitude. It's calculated by grouping the number of digits into sets of three
			using the formula:
		</p>
		<pre class="pre">stepLevel = floor((digits - 1) / 3)</pre>
		<p>Here's how step levels operate across different ranges:</p>
		<dl class="space-y-4">
			<div>
				<dt class="font-bold">Level 0 (1-3 digits)</dt>
				<dd class="opacity-60">
					<dl class="space-y-1">
						<div>
							<dt class="font-semibold">Digit Range:</dt>
							<dd>1 to 999</dd>
						</div>
						<div>
							<dt class="font-semibold">Step Size:</dt>
							<dd>100<sup>0</sup> = <strong>1</strong></dd>
						</div>
					</dl>
				</dd>
			</div>
			<div>
				<dt class="font-bold">Level 1 (4-6 digits)</dt>
				<dd class="opacity-60">
					<dl class="space-y-1">
						<div>
							<dt class="font-semibold">Digit Range:</dt>
							<dd>1,000 to 999,999</dd>
						</div>
						<div>
							<dt class="font-semibold">Step Size:</dt>
							<dd>100<sup>1</sup> = <strong>100</strong></dd>
						</div>
					</dl>
				</dd>
			</div>
			<div>
				<dt class="font-bold">Level 2 and Beyond</dt>
				<dd class="opacity-60">
					Each subsequent level increases the Step Size by a factor of 100 for every additional set
					of three digits.
				</dd>
			</div>
		</dl>
	</section>

	<section class="space-y-6">
		<h4 class="h4">What Happens When One Googol Is Reached?</h4>
		<p>
			Then you've reached a number where it would take 3,1709791984×10<sup>92</sup> Years when every
			second its incremented by one. A number that big it's over the Atom count in the visible
			universe by 10<sup>20</sup>. A number which Google inherited their name from. A number that is
			impossible to be reached under normal circumstances! Isn't that enough?
		</p>
	</section>
</div>
