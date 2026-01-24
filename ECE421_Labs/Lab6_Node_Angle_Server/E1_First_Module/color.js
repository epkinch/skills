class Color {
    constructor(name, code) {
        this.name = name;
        this.code = code;
    }
}

const allColors = [new Color('White', ' #FFFFFF'), new Color('Black', ' #000000'), new Color('Red', ' #FF0000'), 
    new Color('Green', ' #008000'), new Color('Blue', ' #0000FF'), new Color('Green', ' #FFFF00')];

exports.getRandomColor = () => {
    return allColors[Math.floor(Math.random() * allColors.length)]
}

exports.allColors = allColors;