async function init(){
    let rustApp = null;

    try {
        rustApp = await import('../pkg')
    } catch (error) {
        console.error(error)
        return
    }

    const input = document.getElementById('upload')
    const fileReader = new FileReader()

    input.addEventListener('change', ()=>{
        fileReader.readAsDataURL(input.files[0])
    })

    fileReader.onloadend = ()=>{
        const base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/,'')

        const dataUrl = rustApp.grayScale(base64)

        const image = document.getElementById('new-img')
        image.setAttribute(
            'src', dataUrl
        )
    }
}

init()