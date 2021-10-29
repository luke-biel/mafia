<script lang="ts">
    import {Route, Router} from "svelte-navigator";
    import Login from "./components/routes/Login.svelte";
    import PrivateRoute from "./components/utils/PrivateRoute.svelte";
    import Game from "./components/routes/Game.svelte";
</script>

<Router>
    <header>
        Mafia
    </header>

    <main>
        <Route path="/">
            <Login/>
        </Route>

        <PrivateRoute let:location path="/game">
            <Game/>
        </PrivateRoute>
    </main>
</Router>

<style>
    header {
        font-size: 2em;
        text-align: center;
    }

    main {
        width: 80%;
        padding: 1em;
        margin: 1em auto;
        border: 1px dashed black;
    }
</style>
