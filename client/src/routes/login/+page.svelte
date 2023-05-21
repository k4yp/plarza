<script>
    let email = '';
    let password = '';
  
    const login = async () => {
        await fetch('http://127.0.0.1:5000/login',{
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                email:email,
                password:password
            })
        })
        .then((res) => {
            if(res.status === 200){
                window.location.href = '/';
                return res.json()
            }
            else{
                throw new Error('Unable to sign up')
            }
        })
    }
</script>

<div class="login">
    <form on:submit|preventDefault={login}>
        <input type="email" placeholder="email" bind:value={email} required>
        <input type="password" placeholder="password" bind:value={password} required>
        <button type="submit">log in</button>
        <button class="forgot" type="submit">forgot password?</button>
    </form>
</div>

<style>
    .login {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 90vh;
    } 
    form {
        width: 400px;
        text-align: center;
    }
    input {
        font-family: "Chivo Mono", monospace;
        margin: 12px;
        text-align: center;
        font-size: 16px;
        border: none;
        outline: none;
        background-color: unset;
    }
    button {
        font-family: "Chivo Mono", monospace;
        color: #0000ff;
        margin: 12px;
        width: 300px;
        font-size: 16px;
        background-color: unset;
        border: none;
    }
    button:hover{
        text-decoration: underline;
        cursor: pointer;
    }
    .forgot{
        margin-top: 64px;
        color: #272727;
        font-size: 14px;
    }
</style>
  