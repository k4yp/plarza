<div class="header">
    <img class="logo" src="plarza.png" alt="logo" >
    <input class="search" placeholder="search...">
    <a href="./login" class="login">Login</a> 
    <a href="./signup" class="signup">Sign Up</a>
</div>

<style>
    .header {
        margin-top: 4px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 10%;
        height: 50px;
        font-family: Inter;
        font-size: 16px;
    }

    .logo {
        height: 75%;
    }

    .search {
        background-color: unset;
        font-family: Inter;
        font-weight: 600;
        font-size: 16px;
        color: #051728;
        width: 25%;
        padding: 5px;
        padding-left: 32px;
        margin: auto;
        border: none;
        background-color: #E1DACB;
        border-radius: 32px;
    }

    .search::placeholder {
        color: #746f64;
    }

    .search:focus {
        outline: none;
    }

    .signup {
        width: 80px;
        height: 32px;
        text-align: center;
        text-decoration: none;
        font-weight:bolder;
        font-size: 16px;
        color: #051728;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color:#7fff00;
        border-radius: 32px;
    }

    .login {
        padding: 8px;
        margin: 16px;
        width: 64px;
        font-weight: 800;
        font-size: 16px;
        color: #051728;
        text-align: center;
        cursor: pointer; 
        text-decoration: none;
    }
</style>