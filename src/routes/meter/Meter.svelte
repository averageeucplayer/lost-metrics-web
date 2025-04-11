<script lang="ts">
    import type { Encounter } from "$lib/types";
    const imageModules = import.meta.glob("$lib/assets/images/classes/*.webp");

    const imageMap = Object.entries(imageModules).reduce((map, [path, loader]) => {
        const id = path.split("/").pop()!.replace('.webp', '');
        
        map[id] = path;
        return map;
    }, {} as Record<string, string>);

    interface Props {
        encounter: Encounter;
    }

	let { encounter }: Props = $props();
    let scale = 0.25; 
</script>

<div class="p-2">
    {#each encounter.participants as participant}
        <div class="flex p-2 w-full relative">
            <img class="w-8" src={imageMap[participant.classId]} alt={participant.className}/>
            <span>{participant.name}</span>
            <div style="transform: scaleX({scale}); transform-origin: left;" class="absolute bg-gray-700 h-8 -z-10 left-0 right-0"></div>
        </div>
        
    {/each}
</div>