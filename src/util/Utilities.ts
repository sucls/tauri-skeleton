/**
 * 排除部分字段
 */
export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>

/**
 * 可展开
 */
export interface ExpandedAble{
    expanded?: boolean;
}


/**
 * demo
 */
type Student = {
	id: string,
	name: string,
	age: number,
	score: number
}
// 排除
type StudentOmit = Omit<Student, 'id'|'name'>;
let s1: StudentOmit = { age: 21, score:99 }
// 选择
type StudentPick = Pick<Student, 'id'|'name'>;
let s2: StudentPick = { id: "1", name: "tom" }