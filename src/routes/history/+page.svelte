<script lang="ts">
    import { getPastEncounters } from "$lib/api";
    import type { Encounter } from "$lib/types";
    import { Pagination } from "@skeletonlabs/skeleton-svelte";
    import { IconArrowLeft, IconArrowRight, IconChevronLeft, IconChevronRight, IconDots, IconSquare1 } from "@tabler/icons-svelte";
    import { onMount } from "svelte";

    interface State {
        page: number;
        pageSize: number;
        pageSizes: number[];
        items: Encounter[];
    }

    let pageState = $state<State>({
        page: 0,
        pageSize: 0,
        pageSizes: [1, 2, 5],
        items: []
    });
    // const slicedSource = $derived((s: SourceData[]) => s.slice((page - 1) * size, page * size));
   
    onMount(() => {
		onLoad()
	});

    async function onLoad() {
        const criteria = {
            pageSize: 10,
            page: 0,

        };
        const result = await getPastEncounters(criteria);

		
    }

    function onPageSizeChange(event: any) {
        pageState.pageSize = event.pageSize
    }

    function onPageChange(event: any) {
        pageState.pageSize = event.page;
    }

    function onSelect(event: HTMLInputElement) {
        // const size = Number(event.currentTarget.value;
    }

</script>

<div class="">
    <section class="space-y-4">
        <!-- Table -->
        <div class="table-wrap">
          <table class="table table-fixed caption-bottom">
            <thead>
              <tr>
                <th>Name</th>
                <th>Classes</th>
                <th>Duration</th>
              </tr>
            </thead>
            <tbody class="[&>tr]:hover:preset-tonal-primary">
              {#each pageState.items as row}
                <tr>
                  <td>{row.name}</td>
                  <td>{row.classes}</td>
                  <td>{row.duration}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
        <!-- Footer -->
        <footer class="flex justify-between">
          <select name="size" id="size" class="select max-w-[150px]" value={pageState.pageSize} onchange={onPageChange}>
            {#each pageState.pageSizes as value}
              <option value={value}>Items {value}</option>
            {/each}
          </select>
          <Pagination
            data={pageState.items}
            onPageChange={onPageChange}
            pageSize={pageState.pageSize}
            onPageSizeChange={onPageSizeChange}
            siblingCount={4}
          >
            {#snippet labelEllipsis()}<IconDots class="size-4" />{/snippet}
            {#snippet labelNext()}<IconArrowRight class="size-4" />{/snippet}
            {#snippet labelPrevious()}<IconArrowLeft class="size-4" />{/snippet}
            {#snippet labelFirst()}<IconChevronLeft class="size-4" />{/snippet}
            {#snippet labelLast()}<IconChevronRight class="size-4" />{/snippet}
          </Pagination>
        </footer>
      </section>
</div>