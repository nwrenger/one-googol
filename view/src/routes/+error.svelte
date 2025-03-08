<script lang="ts">
	import { page } from '$app/state';
</script>

<h2 class="h2">
	{page.status} - {page.error?.message || 'Unknown Error'}
</h2>
