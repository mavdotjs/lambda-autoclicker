<script lang=ts>
    import "../app.pcss"
    import { appWindow } from "@tauri-apps/api/window"
    import { getVersion } from "@tauri-apps/api/app"
	import { onMount } from "svelte";
    const { children } = $props()
    let version = $state()
    onMount(async () => {
        version = await getVersion()
        await appWindow.setTitle(`${await appWindow.title()} v${version}`)
    })
</script>


<div class="!h-full w-full flex flex-col select-none pt-1">
    <nav class="flex items-center mx-2">
        <!-- svelte-ignore a11y_interactive_supports_focus -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="flex-grow flex justify-between items-center">
            <div class="flex-grow flex justify-between" data-tauri-drag-region  onmousedown={() => appWindow.startDragging()}>
                <div class="font-appbartop"> <abbr title="Lambda">λ</abbr> Autoclicker <span class="text-slate-500 font-sans lowercase">v{version}</span></div>
            </div>
            <div id="window-tools" class='join'>
                <button class="btn btn-xs btn-square join-item hover:text-amber-300" onclick={() => appWindow.minimize()}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14" />
                    </svg>                                   
                </button>
                <button class="btn btn-xs btn-square join-item hover:text-red-300" onclick={() => appWindow.close()}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>
    </nav>
    <main class="flex-grow mx-2 mt-1 mb-2 h-fit">{@render children()}</main>
</div>