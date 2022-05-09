interface ThemeInterface {
  colors: {
    background: string;
    color: string;
    error: string;
  };
  pages: {
    index: {
      cards: {
        signin: string;
        register: string;
      };
      title: {
        a: string;
      };
    };
  };
  footer: {
    borderTop: string;
  };
}

export default ThemeInterface
