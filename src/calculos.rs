fn sumar(x: i32, y: i32) -> i32 {
    let suma = x + y;
    suma
}

fn restar(x: i32, y: i32) -> i32 {
    let resta = x - y;
    resta
}

fn multiplicar(x: i32, y: i32) -> i32 {
    let multiplicacion = x * y;
    multiplicacion
}

fn dividir(x: i32, y: i32) -> i32 {
    let division = x / y;
    division
}    

pub fn evaluar(expresion: &str) -> Result<i32, String> {
    let expresion = expresion.trim();
    if let Some(pos) = expresion.rfind(" + ") {
        let x = expresion[..pos].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        return Ok(sumar(x, y));
    }
    if let Some(pos) = expresion.rfind(" - ") {
        let x = expresion[..pos].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        return Ok(restar(x, y));
    }
    if let Some(pos) = expresion.rfind(" * ") {
        let x = expresion[..pos].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        return Ok(multiplicar(x, y));
    }
    if let Some(pos) = expresion.rfind(" / ") {
        let x = expresion[..pos].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i32>().map_err(|_| "Número inválido".to_string())?;
        if y == 0 { return Err("División por cero".to_string()); }
        return Ok(dividir(x, y));
    }

    Err("Expresión inválida. Usá: 10 + 5".to_string())
}
