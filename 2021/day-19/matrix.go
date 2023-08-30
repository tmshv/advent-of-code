package main

import "math"

var (
	v11 = 0
	v12 = 1
	v13 = 2
	v14 = 3
	v21 = 4
	v22 = 5
	v23 = 6
	v24 = 7
	v31 = 8
	v32 = 9
	v33 = 10
	v34 = 11
	v41 = 12
	v42 = 13
	v43 = 14
	v44 = 15
)

type Matrix3D struct {
	v [16]float64
}

func NewMatrix3D(m [16]float64) *Matrix3D {
	// 	v11, v12, v13, v14,
	// 	v21, v22, v23, v24,
	// 	v31, v32, v33, v34,
	// 	v41, v42, v43, v44 float64,
	// ) *Matrix3D {
	// return &Matrix3D{
	// 	v11, v12, v13, v14,
	// 	v21, v22, v23, v24,
	// 	v31, v32, v33, v34,
	// 	v41, v42, v43, v44,
	// }
	return &Matrix3D{m}
	// return &Matrix3D{
	// 	m[0], m[1], m[2], m[3],
	// 	m[4], m[5], m[6], m[7],
	// 	m[8], m[9], m[10], m[11],
	// 	m[12], m[13], m[14], m[15],
	// }
}

func NewIdentity() *Matrix3D {
	return &Matrix3D{
		[16]float64{
			1, 0, 0, 0,
			0, 1, 0, 0,
			0, 0, 1, 0,
			0, 0, 0, 1,
		},
	}
}

func (m *Matrix3D) Determinant() float64 {
	t1 := m.v[v11] * m.v[v22] * m.v[v33] * m.v[v44]
	t2 := m.v[v11] * m.v[v23] * m.v[v34] * m.v[v42]
	t3 := m.v[v11] * m.v[v24] * m.v[v32] * m.v[v43]
	t4 := m.v[v11] * m.v[v24] * m.v[v33] * m.v[v42]
	t5 := m.v[v11] * m.v[v23] * m.v[v32] * m.v[v44]
	t6 := m.v[v11] * m.v[v22] * m.v[v34] * m.v[v43]
	t7 := m.v[v12] * m.v[v21] * m.v[v33] * m.v[v44]
	t8 := m.v[v13] * m.v[v21] * m.v[v34] * m.v[v42]
	t9 := m.v[v14] * m.v[v21] * m.v[v32] * m.v[v43]
	t10 := m.v[v14] * m.v[v21] * m.v[v33] * m.v[v42]
	t11 := m.v[v13] * m.v[v21] * m.v[v32] * m.v[v41]
	t12 := m.v[v12] * m.v[v21] * m.v[v34] * m.v[v43]
	t13 := m.v[v12] * m.v[v23] * m.v[v31] * m.v[v44]
	t14 := m.v[v13] * m.v[v24] * m.v[v31] * m.v[v42]
	t15 := m.v[v14] * m.v[v22] * m.v[v31] * m.v[v43]
	t16 := m.v[v14] * m.v[v23] * m.v[v31] * m.v[v42]
	t17 := m.v[v13] * m.v[v22] * m.v[v31] * m.v[v44]
	t18 := m.v[v12] * m.v[v24] * m.v[v31] * m.v[v43]
	t19 := m.v[v12] * m.v[v23] * m.v[v34] * m.v[v41]
	t20 := m.v[v13] * m.v[v24] * m.v[v32] * m.v[v41]
	t21 := m.v[v14] * m.v[v22] * m.v[v33] * m.v[v41]
	t22 := m.v[v14] * m.v[v23] * m.v[v32] * m.v[v41]
	t23 := m.v[v13] * m.v[v22] * m.v[v34] * m.v[v41]
	t24 := m.v[v12] * m.v[v24] * m.v[v33] * m.v[v41]

	return t1 + t2 + t3 - t4 - t5 - t6 - t7 - t8 - t9 + t10 + t11 + t12 + t13 + t14 + t15 - t16 - t17 - t18 - t19 - t20 - t21 + t22 + t23 + t24
}

func (m *Matrix3D) Multiply(matrix *Matrix3D) {
	t11 := m.v[v11]*matrix.v[v11] + m.v[v12]*matrix.v[v21] + m.v[v13]*matrix.v[v31] + m.v[v14]*matrix.v[v41]
	t12 := m.v[v11]*matrix.v[v12] + m.v[v12]*matrix.v[v22] + m.v[v13]*matrix.v[v32] + m.v[v14]*matrix.v[v42]
	t13 := m.v[v11]*matrix.v[v13] + m.v[v12]*matrix.v[v23] + m.v[v13]*matrix.v[v33] + m.v[v14]*matrix.v[v43]
	t14 := m.v[v11]*matrix.v[v14] + m.v[v12]*matrix.v[v24] + m.v[v13]*matrix.v[v34] + m.v[v14]*matrix.v[v44]

	t21 := m.v[v21]*matrix.v[v11] + m.v[v22]*matrix.v[v21] + m.v[v23]*matrix.v[v31] + m.v[v24]*matrix.v[v41]
	t22 := m.v[v21]*matrix.v[v12] + m.v[v22]*matrix.v[v22] + m.v[v23]*matrix.v[v32] + m.v[v24]*matrix.v[v42]
	t23 := m.v[v21]*matrix.v[v13] + m.v[v22]*matrix.v[v23] + m.v[v23]*matrix.v[v33] + m.v[v24]*matrix.v[v43]
	t24 := m.v[v21]*matrix.v[v14] + m.v[v22]*matrix.v[v24] + m.v[v23]*matrix.v[v34] + m.v[v24]*matrix.v[v44]

	t31 := m.v[v31]*matrix.v[v11] + m.v[v32]*matrix.v[v21] + m.v[v33]*matrix.v[v31] + m.v[v34]*matrix.v[v41]
	t32 := m.v[v31]*matrix.v[v12] + m.v[v32]*matrix.v[v22] + m.v[v33]*matrix.v[v32] + m.v[v34]*matrix.v[v42]
	t33 := m.v[v31]*matrix.v[v13] + m.v[v32]*matrix.v[v23] + m.v[v33]*matrix.v[v33] + m.v[v34]*matrix.v[v43]
	t34 := m.v[v31]*matrix.v[v14] + m.v[v32]*matrix.v[v24] + m.v[v33]*matrix.v[v34] + m.v[v34]*matrix.v[v44]

	t41 := m.v[v41]*matrix.v[v11] + m.v[v42]*matrix.v[v21] + m.v[v43]*matrix.v[v31] + m.v[v44]*matrix.v[v41]
	t42 := m.v[v41]*matrix.v[v12] + m.v[v42]*matrix.v[v22] + m.v[v43]*matrix.v[v32] + m.v[v44]*matrix.v[v42]
	t43 := m.v[v41]*matrix.v[v13] + m.v[v42]*matrix.v[v23] + m.v[v43]*matrix.v[v33] + m.v[v44]*matrix.v[v43]
	t44 := m.v[v41]*matrix.v[v14] + m.v[v42]*matrix.v[v24] + m.v[v43]*matrix.v[v34] + m.v[v44]*matrix.v[v44]

	m.v[v11] = t11
	m.v[v12] = t12
	m.v[v13] = t13
	m.v[v14] = t14
	m.v[v21] = t21
	m.v[v22] = t22
	m.v[v23] = t23
	m.v[v24] = t24
	m.v[v31] = t31
	m.v[v32] = t32
	m.v[v33] = t33
	m.v[v34] = t34
	m.v[v41] = t41
	m.v[v42] = t42
	m.v[v43] = t43
	m.v[v44] = t44
}

func (m *Matrix3D) Translate(x, y, z float64) {
	matrix := Matrix3D{
		[16]float64{
			1, 0, 0, 0,
			0, 1, 0, 0,
			0, 0, 1, 0,
			x, y, z, 1,
		},
	}
	m.Multiply(&matrix)
}

func (m *Matrix3D) Scale(x, y, z float64) {
	matrix := Matrix3D{
		[16]float64{
			x, 0, 0, 0,
			0, y, 0, 0,
			0, 0, z, 0,
			0, 0, 0, 1,
		},
	}
	m.Multiply(&matrix)
}

func (m *Matrix3D) RotateX(angle float64) {
	s := math.Sin(angle)
	c := math.Cos(angle)
	matrix := Matrix3D{
		[16]float64{
			1, 0, 0, 0,
			0, c, s, 0,
			0, -s, c, 0,
			0, 0, 0, 1,
		},
	}
	m.Multiply(&matrix)
}

func (m *Matrix3D) RotateY(angle float64) {
	s := math.Sin(angle)
	c := math.Cos(angle)
	matrix := Matrix3D{
		[16]float64{
			c, 0, -s, 0,
			0, 1, 0, 0,
			s, 0, c, 0,
			0, 0, 0, 1,
		},
	}
	m.Multiply(&matrix)
}

func (m *Matrix3D) RotateZ(angle float64) {
	s := math.Sin(angle)
	c := math.Cos(angle)
	matrix := Matrix3D{
		[16]float64{
			c, s, 0, 0,
			-s, c, 0, 0,
			0, 0, 1, 0,
			0, 0, 0, 1,
		},
	}
	m.Multiply(&matrix)
}
