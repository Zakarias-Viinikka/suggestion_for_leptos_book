window.onload = () => {
  const event = new CustomEvent('obey_me_rust', {
      detail: String(result)
  });
  window.dispatchEvent(event);
};
