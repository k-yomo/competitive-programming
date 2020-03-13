package util

type UnionFind struct {
	roots []int
}

func NewUnionFind(n int) *UnionFind {
	u := &UnionFind{}
	for i := 0; i < n; i++ {
		u.roots = append(u.roots, i)
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
	u.roots[x] = y
}

func (u *UnionFind) Same(x, y int) bool {
	return u.RootOf(x) == u.RootOf(y)
}
