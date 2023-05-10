<script>
    let email = '';
    let password = '';
  
    function handleSubmit() {
      console.log('email:', email);
      console.log('password:', password);
    }
</script>

<div class="signup">
    <form on:submit|preventDefault={handleSubmit}>
        <input type="email" placeholder="email" bind:value={email} required>
        <input type="password" placeholder="password" bind:value={password} required>
        <button type="submit">sign up</button>
    </form>
</div>

<style>
    .signup {
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
        background-color:unset;
    }
    button {
        font-family: "Chivo Mono", monospace;
        color: #0000ff;
        margin: 12px;
        width: 300px;
        font-size: 16px;
        background-color:unset;
        border: none;
    }
    button:hover{
        text-decoration: underline;
        cursor: pointer;
    }
</style>
  