<script lang="ts">
    import { useEncounterMonitor } from "$lib/store/encounter";
	import { useProcessWatcher, useUpdateManager } from "$lib/store/managers";

	const monitor = useEncounterMonitor();

	const {
		result: updateCheckResult
	} = useUpdateManager();

	const {
		result: processWatcherResult
	} = useProcessWatcher();

</script>

<div class="p-2 overflow-scroll h-screen">
	<div>
		<span>Process Watcher</span>
		<pre><code>{JSON.stringify($processWatcherResult, null, 2)}</code></pre>
	</div>
	<div>
		<span>Updater</span>
		<pre><code>{JSON.stringify($updateCheckResult, null, 2)}</code></pre>
	</div>
	<div>
		<span>Encounter Monitor</span>
		<pre><code>{JSON.stringify($monitor, null, 2)}</code></pre>
	</div>
</div>