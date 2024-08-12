<script lang=ts>
    import { getCurrentWebviewWindow, getAllWebviewWindows } from "@tauri-apps/api/webviewWindow"
    import { currentMonitor } from "@tauri-apps/api/window"
	import { onMount } from "svelte";

    onMount(async () => {
        const screenSize = (await currentMonitor())?.size!
        await getCurrentWebviewWindow().setSize(screenSize)
    })
</script>

<div class="cursor-move !h-full w-full grid grid-rows-3 select-none justify-center items-center text-base-content bg-base-100/30">
    <div></div>
    <span class="hover:opacity-10 transition text-center contents-end">
        Click anywhere, or Press K to select a position.<br />
        ESC to cancel.
    </span>
    <div></div>
</div>

<svelte:window onclick={() => console.log("click")} />