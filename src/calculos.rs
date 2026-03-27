fn sumar(x: i64, y: i64) -> i64 {
    let suma = x + y;
    suma
}

fn restar(x: i64, y: i64) -> i64 {
    let resta = x - y;
    resta
}

fn multiplicar(x: i64, y: i64) -> i64 {
    let multiplicacion = x * y;
    multiplicacion
}

fn dividir(x: i64, y: i64) -> i64 {
    let division = x / y;
    division
}

fn potencia(x: i64, y: i64) -> i64 {
    let mut resultado = 1;
    for _i in 0..y {
        resultado *= x;
    }
    resultado
}

pub fn evaluar(expresion: &str) -> Result<i64, String> {
    let expresion = expresion.trim();
    if let Some(pos) = expresion.rfind(" + ") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(sumar(x, y));
    }
    
    if let Some(pos) = expresion.rfind("+") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 1..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(sumar(x, y));
    }
    
    if let Some(pos) = expresion.rfind(" - ") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(restar(x, y));
    }
    
    if let Some(pos) = expresion.rfind("-") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 1..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(restar(x, y));
    }
    if let Some(pos) = expresion.rfind(" * ") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(multiplicar(x, y));
    }

    if let Some(pos) = expresion.rfind("*") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 1..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(multiplicar(x, y));
    }

    if let Some(pos) = expresion.rfind(" / ") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        if y == 0 { return Err("División por cero".to_string()); }
        return Ok(dividir(x, y));
    }

    if let Some(pos) = expresion.rfind("/") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 1..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        if y == 0 { return Err("División por cero".to_string()); }
        return Ok(dividir(x, y));
    }

    let expresion = expresion.trim();
    if let Some(pos) = expresion.rfind(" ^ ") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 3..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(potencia(x, y));
    }

    let expresion = expresion.trim();
    if let Some(pos) = expresion.rfind("^") {
        let x = expresion[..pos].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        let y = expresion[pos + 1..].trim().parse::<i64>().map_err(|_| "Número inválido".to_string())?;
        return Ok(potencia(x, y));
    }
    
    Err("Expresión inválida. Usá: 10 + 5".to_string())
}
