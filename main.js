import init, { design_new, design_palette, design_load_palette, design_optimize_palette, design_load_image, design_dimensions, design_generate } from './pkg/nhdesigngen.js';

async function run() {
    await init();

    const design = design_new();

    // Update the entire interface
    function update() {
        const palette = design_palette(design);

        // Clear out the palette table
        const palette_table = document.getElementById('palette');
        palette_table.textContent = '';
        const index_row = document.createElement('tr');
        const color_row = document.createElement('tr');
        const hue_row = document.createElement('tr');
        const saturation_row = document.createElement('tr');
        const value_row = document.createElement('tr');

        palette_table.appendChild(index_row);
        palette_table.appendChild(color_row);
        palette_table.appendChild(hue_row);
        palette_table.appendChild(saturation_row);
        palette_table.appendChild(value_row);

        const index_th = document.createElement('th');
        index_th.textContent = 'index';
        index_row.appendChild(index_th);

        const color_th = document.createElement('th');
        color_th.textContent = 'color';
        color_row.appendChild(color_th);

        const hue_th = document.createElement('th');
        hue_th.textContent = 'hue';
        hue_row.appendChild(hue_th);

        const saturation_th = document.createElement('th');
        saturation_th.textContent = 'saturation';
        saturation_row.appendChild(saturation_th);

        const value_th = document.createElement('th');
        value_th.textContent = 'value';
        value_row.appendChild(value_th);

        // Actually add the palette
        palette.forEach((item, index) => {
            const index_td = document.createElement('th');
            index_td.textContent = index + 1;
            index_row.appendChild(index_td);

            const color_td = document.createElement('td');
            color_td.setAttribute('style', `background-color: rgba(${item.r}, ${item.g}, ${item.b}, ${item.a});`);
            color_row.appendChild(color_td);

            if (item.a !== 0) {
                const hue_td = document.createElement('td');
                hue_td.textContent = item.h;
                hue_row.appendChild(hue_td);

                const saturation_td = document.createElement('td');
                saturation_td.textContent = item.s;
                saturation_row.appendChild(saturation_td);

                const value_td = document.createElement('td');
                value_td.textContent = item.v;
                value_row.appendChild(value_td);
            }
        });

        const ditherer = document.getElementById('ditherer').value;

        const dimensions = design_dimensions(design);
        const width = dimensions[0];
        const height = dimensions[1];

        const pixels = design_generate(design, ditherer);

        const design_table = document.getElementById('design');
        design_table.textContent = '';

        const design_header = document.createElement('tr');
        design_table.appendChild(design_header);

        design_header.appendChild(document.createElement('th'));

        // Set up header row
        for (let i = 0; i < width; ++i) {
            const item = document.createElement('th');
            item.textContent = i + 1;
            design_header.appendChild(item);
        }

        const rows = [];
        // Set up body columns
        for (let i = 0; i < height; ++i) {
            const row = document.createElement('tr');

            const item = document.createElement('th');
            item.textContent = i + 1;
            row.appendChild(item);
            rows[i] = row;
            design_table.appendChild(row);
        }

        // Set up the actual pixels
        pixels.forEach((pixel, index) => {
            const y = Math.floor(index / width);
            const row = rows[y];
            const td = document.createElement('td');
            td.textContent = pixel + 1;
            row.appendChild(td);
            const color = palette[pixel];
            const text_color = (color.v > 8 ? 'black' : 'white');
            td.setAttribute('style', `background-color: rgba(${color.r}, ${color.g}, ${color.b}, ${color.a}); color: ${text_color}`);
        });
    }


    // Get a promise for reading the file
    function loadFile(file) {
        return new Promise(resolve => {
            const reader = new FileReader();
            const type = file.type;
            reader.onload = (event) => {
                // Draw the image onto a canvas
                const data = new Uint8Array(event.target.result);
                const blob = new Blob([data], {'type': type});
                const object_url = URL.createObjectURL(blob);
                const img = new Image();
                img.onload = () => {
                    const canvas = document.createElement('canvas');
                    canvas.width = img.width;
                    canvas.height = img.height;
                    const ctx = canvas.getContext('2d');
                    ctx.drawImage(img, 0, 0);
                    const imgData = Array.from(ctx.getImageData(0, 0, img.width, img.height).data);
                    const output = [];
                    for (let i = 0; i < imgData.length; i += 4) {
                        output.push([imgData[i], imgData[i + 1], imgData[i + 2], imgData[i + 3]]);
                    }
                    // Return the canvas rgba data
                    resolve({
                        data: output,
                        width: img.width,
                        height: img.height,
                    });
                }
                img.src = object_url;
            };
            reader.readAsArrayBuffer(file);
        });
    }

    function optimizePalette() {
        const optimizer = document.getElementById('optimizer').value;
        design_optimize_palette(design, optimizer);
    }

    async function setPaletteFiles(event) {
        try {
            // Load all the files into data
            const data = (await Promise.all(Array.from(event.target.files).map(file => loadFile(file)))).map(image => image.data);
            design_load_palette(design, data);
            optimizePalette();
            update();
        } catch (e) {
            alert(e);
        }
    }

    function changeOptimizer(event) {
        optimizePalette();
        update();
    }

    async function setImage(event) {
        try {
            if (event.target.files.length > 0) {
                // Load all the files into data
                const file = await loadFile(event.target.files[0]);
                design_load_image(design, file.data, file.width, file.height);
                update();
            }
        } catch (e) {
            alert(e);
        }
    }

    document.getElementById('loadpalette').addEventListener('change', setPaletteFiles, false);
    document.getElementById('optimizer').addEventListener('change', changeOptimizer, false);
    document.getElementById('ditherer').addEventListener('change', update, false);
    document.getElementById('loadimage').addEventListener('change', setImage, false);

    update();
}

run();
