<script lang="ts">
	import gameNotRunning from '$lib/assets/images/emotes/nineveh_tired.png';
    import gameIcon from '$lib/assets/images/loa.png';

</script>

<div class="flex flex-col">
    <img alt="The game is not running" src={gameNotRunning} />
    <div class="flex items-center">
        <img class="mr-2 h-8 w-8" alt="loa" src={gameIcon} />
        <span>is not running</span>
    </div>
</div>