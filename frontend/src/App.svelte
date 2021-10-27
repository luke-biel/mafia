<script lang="ts">
    import {Route, Router} from "svelte-navigator";
    import Login from "./Login.svelte";
    import PrivateRoute from "./utils/PrivateRoute.svelte";
    import Game from "./Game.svelte";
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
        font-size: 2em
    }
</style>
