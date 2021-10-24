<script lang="ts">
    import {useLocation, useNavigate} from "svelte-navigator";
    import {host, user} from "./stores";
    import {getCookie} from "./cookies";
    import {onMount} from "svelte";

    const navigate = useNavigate();
    const location = useLocation();

    let name;
    let hostname;

    async function onLoginSubmit() {
        const res = await fetch(`${hostname}/register`, {
            method: 'POST',
            mode: 'cors',
            credentials: 'include',
            cache: 'no-cache',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name,
            })
        })
        const {guid} = await res.json()

        $host = hostname
        document.cookie = `mafia-host=${hostname}`
        $user = {name, guid}
        const from = ($location.state && $location.state.from) || "/game";
        navigate(from, {replace: true});
    }

    async function onAppLoad() {
        const guidC = getCookie('mafia-guid')
        const hostC = getCookie('mafia-host')

        if (guidC && hostC) {
            let res = await fetch(`${hostC}/user/${guidC}`, {
                method: 'GET',
                mode: 'cors',
                credentials: 'include',
                cache: 'no-cache'
            })

            if (res.status == 200) {
                const {name} = await res.json()

                $host = hostC
                $user = {name, guid: guidC}
                const from = ($location.state && $location.state.from) || "/game";
                navigate(from, {replace: true});
            }
        }
    }

    onMount(onAppLoad)
</script>

<form on:submit|preventDefault={onLoginSubmit}>
    <input bind:value={hostname} name="host" placeholder="http://192.168.0.25:5069" type="text"/>
    <input bind:value={name} name="name" placeholder="John Doe" type="text"/>
    <br/>
    <button type="submit">Join Game</button>
</form>
