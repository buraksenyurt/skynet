import freedom from 'ic:canisters/freedom';

freedom.sayHello(window.prompt("En sevdiğin renk")).then(lovelyColor => {
  window.alert(lovelyColor);
});
