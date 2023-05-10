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
        <button type="submit">login</button>
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
        margin: auto;
        text-align: center;
    }
    input {
        margin: 12px;
        text-align: center;
        font-size: 16px;
        border: none;
        outline: none;
        background-color:unset;
        font-family: "Chivo Mono", monospace;
    }
    button {
        color: blue;
        font-family: "Chivo Mono", monospace;
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
  