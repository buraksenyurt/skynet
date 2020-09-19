import algebra from 'ic:canisters/algebra';

algebra.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
