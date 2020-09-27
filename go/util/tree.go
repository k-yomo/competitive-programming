package util

type UnionFind struct {
	roots []int
	sizes []int
}

func NewUnionFind(n int) *UnionFind {
	u := &UnionFind{}
	for i := 0; i < n; i++ {
		u.roots = append(u.roots, i)
		u.sizes = append(u.sizes, 1)
	}
	return u
}

func (u *UnionFind) RootOf(i int) int {
	if u.roots[i] == i {
		return i
	} else {
		u.roots[i] = u.RootOf(u.roots[i])
		return u.roots[i]
	}
}

func (u *UnionFind) Unite(x, y int) {
	rootX := u.RootOf(x)
	rootY := u.RootOf(y)
	if rootX == rootY {
		return
	}
	u.sizes[rootX] += u.sizes[rootY]
	u.roots[rootY] = rootX
}

func (u *UnionFind) Same(x, y int) bool {
	return u.RootOf(x) == u.RootOf(y)
}

func (u *UnionFind) Size(x int) int {
	return u.sizes[u.RootOf(x)]
}

