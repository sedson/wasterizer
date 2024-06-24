export async function load_obj(path) {
  const res = await fetch(path);
  const text = await res.text();

  const positions = [];
  const faces = []

  const lines = text.split("\n").map(ln => ln.trim());

  for (const line of lines) {

    const parts = line.split(' ');
    const type = parts[0];
    const data = parts.slice(1);

    switch (type) {

    case 'v':
      // Vertex position
      positions.push(data.map(Number));
      break;


    case 'o':
      // Object label
      break;


    case '#':
      // Comment
      break;

    case 'f':
      // Face
      const verts = data.map(e => e.split('/').map(Number));
      if (verts.length === 3) {
        faces.push(verts.map(v => v[0] - 1));
      } else {
        // Handle quads and ngons?
      }
      break;
    }
  }
  return { positions, faces };
}