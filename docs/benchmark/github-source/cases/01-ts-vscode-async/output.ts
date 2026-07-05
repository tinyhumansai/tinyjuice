/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import { CancellationToken, CancellationTokenSource } from './cancellation.js';
import { BugIndicatingError, CancellationError } from './errors.js';
import { Emitter, Event } from './event.js';
import { Disposable, DisposableMap, DisposableStore, IDisposable, MutableDisposable, toDisposable } from './lifecycle.js';
import { extUri as defaultExtUri, IExtUri } from './resources.js';
import { URI } from './uri.js';
import { setTimeout0 } from './platform.js';
import { MicrotaskDelay } from './symbols.js';
import { Lazy } from './lazy.js';

export function isThenable<T>(obj: unknown): obj is Promise<T> {
	return !!obj && typeof (obj as unknown as Promise<T>).then === 'function';
}

export interface CancelablePromise<T> extends Promise<T> {
	cancel(): void;
}

export function createCancelablePromise<T>(callback: (token: CancellationToken) => Promise<T>): CancelablePromise<T> { … 36 line(s) … ⟦tj:909236ff0c71c58ac77dba1335f62bff⟧ }

/**
 * Returns a promise that resolves with `undefined` as soon as the passed token is cancelled.
 * @see {@link raceCancellationError}
 */
export function raceCancellation<T>(promise: Promise<T>, token: CancellationToken): Promise<T | undefined>;

/**
 * Returns a promise that resolves with `defaultValue` as soon as the passed token is cancelled.
 * @see {@link raceCancellationError}
 */
export function raceCancellation<T>(promise: Promise<T>, token: CancellationToken, defaultValue: T): Promise<T>;

export function raceCancellation<T>(promise: Promise<T>, token: CancellationToken, defaultValue?: T): Promise<T | undefined> { … 9 line(s) … ⟦tj:348bce64c42af1cfa2aec6ea583d6218⟧ }

/**
 * Returns a promise that rejects with an {@CancellationError} as soon as the passed token is cancelled.
 * @see {@link raceCancellation}
 */
export function raceCancellationError<T>(promise: Promise<T>, token: CancellationToken): Promise<T> { … 9 line(s) … ⟦tj:3ed4490a0c3aa44ad339bb9893f49116⟧ }

/**
 * Returns as soon as one of the promises resolves or rejects and cancels remaining promises
 */
export async function raceCancellablePromises<T>(cancellablePromises: CancelablePromise<T>[]): Promise<T> { … 14 line(s) … ⟦tj:d756fa0ef42cf47ee06d65582cb5648f⟧ }

export function raceTimeout<T>(promise: Promise<T>, timeout: number, onTimeout?: () => void): Promise<T | undefined> { … 13 line(s) … ⟦tj:9743417949ec92f74b2556bc19c816ab⟧ }

export function raceFilter<T>(promises: Promise<T>[], filter: (result: T) => boolean): Promise<T | undefined> { … 25 line(s) … ⟦tj:27a3a7dd73b1e7c845f0bdc2e5529caa⟧ }

export function asPromise<T>(callback: () => T | Thenable<T>): Promise<T> { … 10 line(s) … ⟦tj:78b588074519a330be64d778849f42df⟧ }

/**
 * Creates and returns a new promise, plus its `resolve` and `reject` callbacks.
 *
 * Replace with standardized [`Promise.withResolvers`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers) once it is supported
 */
export function promiseWithResolvers<T>(): { promise: Promise<T>; resolve: (value: T | PromiseLike<T>) => void; reject: (err?: any) => void } { … 9 line(s) … ⟦tj:ad7082bd451fb4b8942635fbf71c0374⟧ }

export interface ITask<T> {
	(): T;
}

/**
 * A helper to prevent accumulation of sequential async tasks.
 *
 * Imagine a mail man with the sole task of delivering letters. As soon as
 * a letter submitted for delivery, he drives to the destination, delivers it
 * and returns to his base. Imagine that during the trip, N more letters were submitted.
 * When the mail man returns, he picks those N letters and delivers them all in a
 * single trip. Even though N+1 submissions occurred, only 2 deliveries were made.
 *
 * The throttler implements this via the queue() method, by providing it a task
 * factory. Following the example:
 *
 * 		const throttler = new Throttler();
 * 		const letters = [];
 *
 * 		function deliver() {
 * 			const lettersToDeliver = letters;
 * 			letters = [];
 * 			return makeTheTrip(lettersToDeliver);
 * 		}
 *
 * 		function onLetterReceived(l) {
 * 			letters.push(l);
 * 			throttler.queue(deliver);
 * 		}
 */
export class Throttler implements IDisposable {

	private activePromise: Promise<any> | null;
	private queuedPromise: Promise<any> | null;
	private queuedPromiseFactory: ITask<Promise<any>> | null;

	private isDisposed = false;

	constructor() { … 5 line(s) … ⟦tj:6f7e80146016e9b7f1651e7b416a53a5⟧ }

	queue<T>(promiseFactory: ITask<Promise<T>>): Promise<T> { … 44 line(s) … ⟦tj:097fa63ec313686ab5e6d8a182735c77⟧ }

	dispose(): void {
		this.isDisposed = true;
	}
}

export class Sequencer {

	private current: Promise<unknown> = Promise.resolve(null);

	queue<T>(promiseTask: ITask<Promise<T>>): Promise<T> {
		return this.current = this.current.then(() => promiseTask(), () => promiseTask());
	}
}

export class SequencerByKey<TKey> {

	private promiseMap = new Map<TKey, Promise<unknown>>();

	queue<T>(key: TKey, promiseTask: ITask<Promise<T>>): Promise<T> { … 13 line(s) … ⟦tj:86bceb52aea2bcc3306b1658c418bc11⟧ }
}

interface IScheduledLater extends IDisposable {
	isTriggered(): boolean;
}

const timeoutDeferred = (timeout: number, fn: () => void): IScheduledLater => { … 14 line(s) … ⟦tj:e5f2c80855cc0be17efb50cbd1bdadba⟧ };

const microtaskDeferred = (fn: () => void): IScheduledLater => { … 14 line(s) … ⟦tj:cf2ccf3f4423e2eaf80b653e523de823⟧ };

/**
 * A helper to delay (debounce) execution of a task that is being requested often.
 *
 * Following the throttler, now imagine the mail man wants to optimize the number of
 * trips proactively. The trip itself can be long, so he decides not to make the trip
 * as soon as a letter is submitted. Instead he waits a while, in case more
 * letters are submitted. After said waiting period, if no letters were submitted, he
 * decides to make the trip. Imagine that N more letters were submitted after the first
 * one, all within a short period of time between each other. Even though N+1
 * submissions occurred, only 1 delivery was made.
 *
 * The delayer offers this behavior via the trigger() method, into which both the task
 * to be executed and the waiting period (delay) must be passed in as arguments. Following
 * the example:
 *
 * 		const delayer = new Delayer(WAITING_PERIOD);
 * 		const letters = [];
 *
 * 		function letterReceived(l) {
 * 			letters.push(l);
 * 			delayer.trigger(() => { return makeTheTrip(); });
 * 		}
 */
export class Delayer<T> implements IDisposable {

	private deferred: IScheduledLater | null;
	private completionPromise: Promise<any> | null;
	private doResolve: ((value?: any | Promise<any>) => void) | null;
	private doReject: ((err: any) => void) | null;
	private task: ITask<T | Promise<T>> | null;

	constructor(public defaultDelay: number | typeof MicrotaskDelay) { … 7 line(s) … ⟦tj:1c7e9eab0a20eeda351a30e4a9377e83⟧ }

	trigger(task: ITask<T | Promise<T>>, delay = this.defaultDelay): Promise<T> { … 29 line(s) … ⟦tj:99ae02a78d41040f9075865dddae109b⟧ }

	isTriggered(): boolean {
		return !!this.deferred?.isTriggered();
	}

	cancel(): void { … 8 line(s) … ⟦tj:0ff105a374d9343cef355daf762bbd16⟧ }

	private cancelTimeout(): void { … 4 line(s) … ⟦tj:ac8d300b41ef2957c26d8c9ba64b873b⟧ }

	dispose(): void {
		this.cancel();
	}
}

/**
 * A helper to delay execution of a task that is being requested often, while
 * preventing accumulation of consecutive executions, while the task runs.
 *
 * The mail man is clever and waits for a certain amount of time, before going
 * out to deliver letters. While the mail man is going out, more letters arrive
 * and can only be delivered once he is back. Once he is back the mail man will
 * do one more trip to deliver the letters that have accumulated while he was out.
 */
export class ThrottledDelayer<T> {

	private delayer: Delayer<Promise<T>>;
	private throttler: Throttler;

	constructor(defaultDelay: number) { … 4 line(s) … ⟦tj:57d288d4ac74845ff5b3cb018dcdbb8d⟧ }

	trigger(promiseFactory: ITask<Promise<T>>, delay?: number): Promise<T> {
		return this.delayer.trigger(() => this.throttler.queue(promiseFactory), delay) as unknown as Promise<T>;
	}

	isTriggered(): boolean {
		return this.delayer.isTriggered();
	}

	cancel(): void {
		this.delayer.cancel();
	}

	dispose(): void { … 4 line(s) … ⟦tj:ccfad34211e7e71850487df9cac15685⟧ }
}

/**
 * A barrier that is initially closed and then becomes opened permanently.
 */
export class Barrier {
	private _isOpen: boolean;
	private _promise: Promise<boolean>;
	private _completePromise!: (v: boolean) => void;

	constructor() { … 6 line(s) … ⟦tj:9adbc719441165a129d25a831078bfa9⟧ }

	isOpen(): boolean {
		return this._isOpen;
	}

	open(): void { … 4 line(s) … ⟦tj:0231f919bc9b84f1f665c17941367392⟧ }

	wait(): Promise<boolean> {
		return this._promise;
	}
}

/**
 * A barrier that is initially closed and then becomes opened permanently after a certain period of
 * time or when open is called explicitly
 */
export class AutoOpenBarrier extends Barrier {

	private readonly _timeout: any;

	constructor(autoOpenTimeMs: number) { … 4 line(s) … ⟦tj:0bf2e36c2f2b271b6c66bd0cb1203741⟧ }

	override open(): void { … 4 line(s) … ⟦tj:ee93571db105fc617e4d188ed6a59b30⟧ }
}

export function timeout(millis: number): CancelablePromise<void>;
export function timeout(millis: number, token: CancellationToken): Promise<void>;
export function timeout(millis: number, token?: CancellationToken): CancelablePromise<void> | Promise<void> { … 17 line(s) … ⟦tj:7e9d89ff7d5318c871cf65e7aed65ce9⟧ }

/**
 * Creates a timeout that can be disposed using its returned value.
 * @param handler The timeout handler.
 * @param timeout An optional timeout in milliseconds.
 * @param store An optional {@link DisposableStore} that will have the timeout disposable managed automatically.
 *
 * @example
 * const store = new DisposableStore;
 * // Call the timeout after 1000ms at which point it will be automatically
 * // evicted from the store.
 * const timeoutDisposable = disposableTimeout(() => {}, 1000, store);
 *
 * if (foo) {
 *   // Cancel the timeout and evict it from store.
 *   timeoutDisposable.dispose();
 * }
 */
export function disposableTimeout(handler: () => void, timeout = 0, store?: DisposableStore): IDisposable { … 14 line(s) … ⟦tj:c5c1f91e6dcf6a8b97587721115b6bd4⟧ }

/**
 * Runs the provided list of promise factories in sequential order. The returned
 * promise will complete to an array of results from each promise.
 */

export function sequence<T>(promiseFactories: ITask<Promise<T>>[]): Promise<T[]> { … 24 line(s) … ⟦tj:309114c3f521b699babcd68e3e411407⟧ }

export function first<T>(promiseFactories: ITask<Promise<T>>[], shouldStop: (t: T) => boolean = t => !!t, defaultValue: T | null = null): Promise<T | null> { … 23 line(s) … ⟦tj:15f3ca378eacd98f8f5e98f31fa84b35⟧ }

/**
 * Returns the result of the first promise that matches the "shouldStop",
 * running all promises in parallel. Supports cancelable promises.
 */
export function firstParallel<T>(promiseList: Promise<T>[], shouldStop?: (t: T) => boolean, defaultValue?: T | null): Promise<T | null>;
export function firstParallel<T, R extends T>(promiseList: Promise<T>[], shouldStop: (t: T) => t is R, defaultValue?: R | null): Promise<R | null>;
export function firstParallel<T>(promiseList: Promise<T>[], shouldStop: (t: T) => boolean = t => !!t, defaultValue: T | null = null) { … 32 line(s) … ⟦tj:bc769d389ec471e6b18bad51d677e95a⟧ }

interface ILimitedTaskFactory<T> {
	factory: ITask<Promise<T>>;
	c: (value: T | Promise<T>) => void;
	e: (error?: unknown) => void;
}

export interface ILimiter<T> {

	readonly size: number;

	queue(factory: ITask<Promise<T>>): Promise<T>;

	clear(): void;
}

/**
 * A helper to queue N promises and run them all with a max degree of parallelism. The helper
 * ensures that at any time no more than M promises are running at the same time.
 */
export class Limiter<T> implements ILimiter<T> {

	private _size = 0;
	private _isDisposed = false;
	private runningPromises: number;
	private readonly maxDegreeOfParalellism: number;
	private readonly outstandingPromises: ILimitedTaskFactory<T>[];
	private readonly _onDrained: Emitter<void>;

	constructor(maxDegreeOfParalellism: number) { … 6 line(s) … ⟦tj:815f52200ca690aaddf905153183ce8e⟧ }

	/**
	 *
	 * @returns A promise that resolved when all work is done (onDrained) or when
	 * there is nothing to do
	 */
	whenIdle(): Promise<void> { … 5 line(s) … ⟦tj:0d8422c2af1965f2303a40042c7d98d0⟧ }

	get onDrained(): Event<void> {
		return this._onDrained.event;
	}

	get size(): number {
		return this._size;
	}

	queue(factory: ITask<Promise<T>>): Promise<T> { … 11 line(s) … ⟦tj:c2f16520bd6606261499720a9536e9a1⟧ }

	private consume(): void { … 10 line(s) … ⟦tj:a382aae0cb8edac1ce03515767451d5f⟧ }

	private consumed(): void { … 13 line(s) … ⟦tj:ff64041b3ec8e161c7409e9536708f72⟧ }

	clear(): void { … 7 line(s) … ⟦tj:077960cf0389f5a86aa538dc7238e371⟧ }

	dispose(): void { … 6 line(s) … ⟦tj:f66cbdb34e34ccd1c3f50e0ac56d1908⟧ }
}

/**
 * A queue is handles one promise at a time and guarantees that at any time only one promise is executing.
 */
export class Queue<T> extends Limiter<T> {

	constructor() {
		super(1);
	}
}

/**
 * Same as `Queue`, ensures that only 1 task is executed at the same time. The difference to `Queue` is that
 * there is only 1 task about to be scheduled next. As such, calling `queue` while a task is executing will
 * replace the currently queued task until it executes.
 *
 * As such, the returned promise may not be from the factory that is passed in but from the next factory that
 * is running after having called `queue`.
 */
export class LimitedQueue {

	private readonly sequentializer = new TaskSequentializer();

	private tasks = 0;

	queue(factory: ITask<Promise<void>>): Promise<void> { … 9 line(s) … ⟦tj:18f31ad272d010b26072e5682c77fabf⟧ }
}

/**
 * A helper to organize queues per resource. The ResourceQueue makes sure to manage queues per resource
 * by disposing them once the queue is empty.
 */
export class ResourceQueue implements IDisposable {

	private readonly queues = new Map<string, Queue<void>>();

	private readonly drainers = new Set<DeferredPromise<void>>();

	private drainListeners: DisposableMap<number> | undefined = undefined;
	private drainListenerCount = 0;

	async whenDrained(): Promise<void> { … 10 line(s) … ⟦tj:1a241bd36e9847205b4d7fae81cce7e1⟧ }

	private isDrained(): boolean { … 9 line(s) … ⟦tj:0eeb55080b90009e9e74dc7a95c35823⟧ }

	queueSize(resource: URI, extUri: IExtUri = defaultExtUri): number { … 5 line(s) … ⟦tj:d29ca710b223725a2a7f3ef1d36be53d⟧ }

	queueFor(resource: URI, factory: ITask<Promise<void>>, extUri: IExtUri = defaultExtUri): Promise<void> { … 30 line(s) … ⟦tj:ca2cf292831a610efd292886bd1ff884⟧ }

	private onDidQueueDrain(): void { … 7 line(s) … ⟦tj:857f41d98074db7a6732d77624763422⟧ }

	private releaseDrainers(): void { … 7 line(s) … ⟦tj:05ad4fd738ef474b23d5874b4ae4c633⟧ }

	dispose(): void { … 17 line(s) … ⟦tj:812cda00cece065f8c241762a83116fd⟧ }
}

export class TimeoutTimer implements IDisposable {
	private _token: any;
	private _isDisposed = false;

	constructor();
	constructor(runner: () => void, timeout: number);
	constructor(runner?: () => void, timeout?: number) { … 7 line(s) … ⟦tj:9c9d6298aed7b90673fbacea11dd61ef⟧ }

	dispose(): void { … 4 line(s) … ⟦tj:6e1a2d79f619b09ab602280acf52f4eb⟧ }

	cancel(): void { … 6 line(s) … ⟦tj:796823543f5ee7b3f1ce967aecbe7cdd⟧ }

	cancelAndSet(runner: () => void, timeout: number): void { … 11 line(s) … ⟦tj:edcd11eb9d5a4201c30aae878e1bd494⟧ }

	setIfNotSet(runner: () => void, timeout: number): void { … 14 line(s) … ⟦tj:b6e3db3d7db576ab906f417cf461304a⟧ }
}

export class IntervalTimer implements IDisposable {

	private disposable: IDisposable | undefined = undefined;
	private isDisposed = false;

	cancel(): void { … 4 line(s) … ⟦tj:dfff5eb2e8949acc9f2250e1d5f36d38⟧ }

	cancelAndSet(runner: () => void, interval: number, context = globalThis): void { … 15 line(s) … ⟦tj:ffed231287be2ac03b050d521582b666⟧ }

	dispose(): void { … 4 line(s) … ⟦tj:b09b96473ea160e7ab562cfb917d1b56⟧ }
}

export class RunOnceScheduler implements IDisposable {

	protected runner: ((...args: unknown[]) => void) | null;

	private timeoutToken: any;
	private timeout: number;
	private timeoutHandler: () => void;

	constructor(runner: (...args: any[]) => void, delay: number) { … 6 line(s) … ⟦tj:cc7b9c9b43e9f94f4b5bc10b4197be74⟧ }

	/**
	 * Dispose RunOnceScheduler
	 */
	dispose(): void { … 4 line(s) … ⟦tj:bcde7f9b8be9e658786507a80aedc73a⟧ }

	/**
	 * Cancel current scheduled runner (if any).
	 */
	cancel(): void { … 6 line(s) … ⟦tj:4829b838fc8644f8ef1803a3e0ff435c⟧ }

	/**
	 * Cancel previous runner (if any) & schedule a new runner.
	 */
	schedule(delay = this.timeout): void { … 4 line(s) … ⟦tj:9efabba0b6c4c7046ecb7e25ffef5b67⟧ }

	get delay(): number {
		return this.timeout;
	}

	set delay(value: number) {
		this.timeout = value;
	}

	/**
	 * Returns true if scheduled.
	 */
	isScheduled(): boolean {
		return this.timeoutToken !== -1;
	}

	flush(): void { … 6 line(s) … ⟦tj:fceb7acfba9ceb8934923a4d51084c94⟧ }

	private onTimeout() { … 6 line(s) … ⟦tj:fdb76b92c632f57c204dd9f37b02458e⟧ }

	protected doRun(): void {
		this.runner?.();
	}
}

/**
 * Same as `RunOnceScheduler`, but doesn't count the time spent in sleep mode.
 * > **NOTE**: Only offers 1s resolution.
 *
 * When calling `setTimeout` with 3hrs, and putting the computer immediately to sleep
 * for 8hrs, `setTimeout` will fire **as soon as the computer wakes from sleep**. But
 * this scheduler will execute 3hrs **after waking the computer from sleep**.
 */
export class ProcessTimeRunOnceScheduler {

	private runner: (() => void) | null;
	private timeout: number;

	private counter: number;
	private intervalToken: any;
	private intervalHandler: () => void;

	constructor(runner: () => void, delay: number) { … 10 line(s) … ⟦tj:7cb90f702084432e3f2c5238e67b78fe⟧ }

	dispose(): void { … 4 line(s) … ⟦tj:bcde7f9b8be9e658786507a80aedc73a⟧ }

	cancel(): void { … 6 line(s) … ⟦tj:34d8aeaaac146f146b1e00141e3ea612⟧ }

	/**
	 * Cancel previous runner (if any) & schedule a new runner.
	 */
	schedule(delay = this.timeout): void { … 8 line(s) … ⟦tj:656f90b14440f19b6bc82b790439339f⟧ }

	/**
	 * Returns true if scheduled.
	 */
	isScheduled(): boolean {
		return this.intervalToken !== -1;
	}

	private onInterval() { … 12 line(s) … ⟦tj:00dffb9b7c65c958cec29dcd826cd189⟧ }
}

export class RunOnceWorker<T> extends RunOnceScheduler {

	private units: T[] = [];

	constructor(runner: (units: T[]) => void, timeout: number) {
		super(runner, timeout);
	}

	work(unit: T): void { … 7 line(s) … ⟦tj:10ba42456c7fca076cbdd994cc25eb0a⟧ }

	protected override doRun(): void { … 6 line(s) … ⟦tj:14fe9ce5153f7a6f057734e8d4f7cdfc⟧ }

	override dispose(): void { … 5 line(s) … ⟦tj:5b3af9de647f577cfed9c06baad3b102⟧ }
}

export interface IThrottledWorkerOptions {

	/**
	 * maximum of units the worker will pass onto handler at once
	 */
	maxWorkChunkSize: number;

	/**
	 * maximum of units the worker will keep in memory for processing
	 */
	maxBufferedWork: number | undefined;

	/**
	 * delay before processing the next round of chunks when chunk size exceeds limits
	 */
	throttleDelay: number;

	/**
	 * When enabled will guarantee that two distinct calls to `work()` are not executed
	 * without throttle delay between them.
	 * Otherwise if the worker isn't currently throttling it will execute work immediately.
	 */
	waitThrottleDelayBetweenWorkUnits?: boolean;
}

/**
 * The `ThrottledWorker` will accept units of work `T`
 * to handle. The contract is:
 * * there is a maximum of units the worker can handle at once (via `maxWorkChunkSize`)
 * * there is a maximum of units the worker will keep in memory for processing (via `maxBufferedWork`)
 * * after having handled `maxWorkChunkSize` units, the worker needs to rest (via `throttleDelay`)
 */
export class ThrottledWorker<T> extends Disposable {

	private readonly pendingWork: T[] = [];

	private readonly throttler = this._register(new MutableDisposable<RunOnceScheduler>());
	private disposed = false;
	private lastExecutionTime = 0;

	constructor(
		private options: IThrottledWorkerOptions,
		private readonly handler: (units: T[]) => void
	) {
		super();
	}

	/**
	 * The number of work units that are pending to be processed.
	 */
	get pending(): number { return this.pendingWork.length; }

	/**
	 * Add units to be worked on. Use `pending` to figure out
	 * how many units are not yet processed after this method
	 * was called.
	 *
	 * @returns whether the work was accepted or not. If the
	 * worker is disposed, it will not accept any more work.
	 * If the number of pending units would become larger
	 * than `maxPendingWork`, more work will also not be accepted.
	 */
	work(units: readonly T[]): boolean { … 44 line(s) … ⟦tj:aa3200230043208bdf56db1622dec4c3⟧ }

	private doWork(): void { … 11 line(s) … ⟦tj:e8256905956158c76b87f84ee0e6daff⟧ }

	private scheduleThrottler(delay = this.options.throttleDelay): void { … 8 line(s) … ⟦tj:332a22f45693bcd2b968b32c06f61302⟧ }

	override dispose(): void { … 5 line(s) … ⟦tj:60b796397a0b04197774114f855d52c3⟧ }
}

//#region -- run on idle tricks ------------

export interface IdleDeadline {
	readonly didTimeout: boolean;
	timeRemaining(): number;
}

type IdleApi = Pick<typeof globalThis, 'requestIdleCallback' | 'cancelIdleCallback'>;


/**
 * Execute the callback the next time the browser is idle, returning an
 * {@link IDisposable} that will cancel the callback when disposed. This wraps
 * [requestIdleCallback] so it will fallback to [setTimeout] if the environment
 * doesn't support it.
 *
 * @param callback The callback to run when idle, this includes an
 * [IdleDeadline] that provides the time alloted for the idle callback by the
 * browser. Not respecting this deadline will result in a degraded user
 * experience.
 * @param timeout A timeout at which point to queue no longer wait for an idle
 * callback but queue it on the regular event loop (like setTimeout). Typically
 * this should not be used.
 *
 * [IdleDeadline]: https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline
 * [requestIdleCallback]: https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
 * [setTimeout]: https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout
 *
 * **Note** that there is `dom.ts#runWhenWindowIdle` which is better suited when running inside a browser
 * context
 */
export let runWhenGlobalIdle: (callback: (idle: IdleDeadline) => void, timeout?: number) => IDisposable;

export let _runWhenIdle: (targetWindow: IdleApi, callback: (idle: IdleDeadline) => void, timeout?: number) => IDisposable;

(function () {
	if (typeof globalThis.requestIdleCallback !== 'function' || typeof globalThis.cancelIdleCallback !== 'function') {
		_runWhenIdle = (_targetWindow, runner, timeout?) => { … 24 line(s) … ⟦tj:a85aca19f34ffb2741e2a942a0a9e457⟧ };
	} else {
		_runWhenIdle = (targetWindow: IdleApi, runner, timeout?) => { … 13 line(s) … ⟦tj:ac8e157fef8ae49cc85e9ccd0fbcfe51⟧ };
	}
	runWhenGlobalIdle = (runner, timeout) => _runWhenIdle(globalThis, runner, timeout);
})();

export abstract class AbstractIdleValue<T> {

	private readonly _executor: () => void;
	private readonly _handle: IDisposable;

	private _didRun: boolean = false;
	private _value?: T;
	private _error: unknown;

	constructor(targetWindow: IdleApi, executor: () => T) { … 12 line(s) … ⟦tj:137aed3a11bca8d09544a2f826e6f69e⟧ }

	dispose(): void {
		this._handle.dispose();
	}

	get value(): T { … 10 line(s) … ⟦tj:f81eb0dd3aaf70bd63a7bdc07faa5830⟧ }

	get isInitialized(): boolean {
		return this._didRun;
	}
}

/**
 * An `IdleValue` that always uses the current window (which might be throttled or inactive)
 *
 * **Note** that there is `dom.ts#WindowIdleValue` which is better suited when running inside a browser
 * context
 */
export class GlobalIdleValue<T> extends AbstractIdleValue<T> {

	constructor(executor: () => T) {
		super(globalThis, executor);
	}
}

//#endregion

export async function retry<T>(task: ITask<Promise<T>>, delay: number, retries: number): Promise<T> { … 15 line(s) … ⟦tj:f5e4185c3794a96a9a1a7d6e72d68a22⟧ }

//#region Task Sequentializer

interface IRunningTask {
	readonly taskId: number;
	readonly cancel: () => void;
	readonly promise: Promise<void>;
}

interface IQueuedTask {
	readonly promise: Promise<void>;
	readonly promiseResolve: () => void;
	readonly promiseReject: (error: Error) => void;
	run: ITask<Promise<void>>;
}

export interface ITaskSequentializerWithRunningTask {
	readonly running: Promise<void>;
}

export interface ITaskSequentializerWithQueuedTask {
	readonly queued: IQueuedTask;
}

/**
 * @deprecated use `LimitedQueue` instead for an easier to use API
 */
export class TaskSequentializer {

	private _running?: IRunningTask;
	private _queued?: IQueuedTask;

	isRunning(taskId?: number): this is ITaskSequentializerWithRunningTask { … 7 line(s) … ⟦tj:0df299fd854867e1536a15e9f5a4911d⟧ }

	get running(): Promise<void> | undefined {
		return this._running?.promise;
	}

	cancelRunning(): void {
		this._running?.cancel();
	}

	run(taskId: number, promise: Promise<void>, onCancel?: () => void,): Promise<void> { … 7 line(s) … ⟦tj:e3033a88385743d28b96094d9148ca78⟧ }

	private doneRunning(taskId: number): void { … 10 line(s) … ⟦tj:ed4c6a948d9da105234dc34cb6018fd6⟧ }

	private runQueued(): void { … 9 line(s) … ⟦tj:ffb28ff063dcfbf57f8d4e262ec54b34⟧ }

	/**
	 * Note: the promise to schedule as next run MUST itself call `run`.
	 *       Otherwise, this sequentializer will report `false` for `isRunning`
	 *       even when this task is running. Missing this detail means that
	 *       suddenly multiple tasks will run in parallel.
	 */
	queue(run: ITask<Promise<void>>): Promise<void> { … 22 line(s) … ⟦tj:33012e007e58d69d2adb0de5979e9286⟧ }

	hasQueued(): this is ITaskSequentializerWithQueuedTask {
		return !!this._queued;
	}

	async join(): Promise<void> {
		return this._queued?.promise ?? this._running?.promise;
	}
}

//#endregion

//#region

/**
 * The `IntervalCounter` allows to count the number
 * of calls to `increment()` over a duration of
 * `interval`. This utility can be used to conditionally
 * throttle a frequent task when a certain threshold
 * is reached.
 */
export class IntervalCounter {

	private lastIncrementTime = 0;

	private value = 0;

	constructor(private readonly interval: number, private readonly nowFn = () => Date.now()) { }

	increment(): number { … 14 line(s) … ⟦tj:f77b6ce7af347a5655aa27580adf4265⟧ }
}

//#endregion

//#region

export type ValueCallback<T = unknown> = (value: T | Promise<T>) => void;

const enum DeferredOutcome {
	Resolved,
	Rejected
}

/**
 * Creates a promise whose resolution or rejection can be controlled imperatively.
 */
export class DeferredPromise<T> {

	private completeCallback!: ValueCallback<T>;
	private errorCallback!: (err: unknown) => void;
	private outcome?: { outcome: DeferredOutcome.Rejected; value: any } | { outcome: DeferredOutcome.Resolved; value: T };

	public get isRejected() {
		return this.outcome?.outcome === DeferredOutcome.Rejected;
	}

	public get isResolved() {
		return this.outcome?.outcome === DeferredOutcome.Resolved;
	}

	public get isSettled() {
		return !!this.outcome;
	}

	public get value() {
		return this.outcome?.outcome === DeferredOutcome.Resolved ? this.outcome?.value : undefined;
	}

	public readonly p: Promise<T>;

	constructor() { … 6 line(s) … ⟦tj:793f7d310ec3e3440e7e5e4409b392f9⟧ }

	public complete(value: T) { … 7 line(s) … ⟦tj:348008875cb326c90c31671a6cd4ebb4⟧ }

	public error(err: unknown) { … 7 line(s) … ⟦tj:a7554994de2a51612b2d390f6ac7675d⟧ }

	public cancel() {
		return this.error(new CancellationError());
	}
}

//#endregion

//#region Promises

export namespace Promises {

	/**
	 * A drop-in replacement for `Promise.all` with the only difference
	 * that the method awaits every promise to either fulfill or reject.
	 *
	 * Similar to `Promise.all`, only the first error will be returned
	 * if any.
	 */
	export async function settled<T>(promises: Promise<T>[]): Promise<T[]> { … 17 line(s) … ⟦tj:6daa5585e387c373d042373135c06871⟧ }

	/**
	 * A helper to create a new `Promise<T>` with a body that is a promise
	 * itself. By default, an error that raises from the async body will
	 * end up as a unhandled rejection, so this utility properly awaits the
	 * body and rejects the promise as a normal promise does without async
	 * body.
	 *
	 * This method should only be used in rare cases where otherwise `async`
	 * cannot be used (e.g. when callbacks are involved that require this).
	 */
	export function withAsyncBody<T, E = Error>(bodyFn: (resolve: (value: T) => unknown, reject: (error: E) => unknown) => Promise<unknown>): Promise<T> { … 10 line(s) … ⟦tj:9ecd9d625a6c0cc5d5edda9cc3d6a371⟧ }
}

export class StatefulPromise<T> {
	private _value: T | undefined = undefined;
	get value(): T | undefined { return this._value; }

	private _error: unknown = undefined;
	get error(): unknown { return this._error; }

	private _isResolved = false;
	get isResolved() { return this._isResolved; }

	public readonly promise: Promise<T>;

	constructor(promise: Promise<T>) { … 14 line(s) … ⟦tj:669a284e0322a6b26980a250707536ca⟧ }

	/**
	 * Returns the resolved value.
	 * Throws if the promise is not resolved yet.
	 */
	public requireValue(): T { … 9 line(s) … ⟦tj:39f5d2688ee4b5aed289340402e9764b⟧ }
}

export class LazyStatefulPromise<T> {
	private readonly _promise = new Lazy(() => new StatefulPromise(this._compute()));

	constructor(
		private readonly _compute: () => Promise<T>,
	) { }

	/**
	 * Returns the resolved value.
	 * Throws if the promise is not resolved yet.
	 */
	public requireValue(): T {
		return this._promise.value.requireValue();
	}

	/**
	 * Returns the promise (and triggers a computation of the promise if not yet done so).
	 */
	public getPromise(): Promise<T> {
		return this._promise.value.promise;
	}

	/**
	 * Reads the current value without triggering a computation of the promise.
	 */
	public get currentValue(): T | undefined {
		return this._promise.rawValue?.value;
	}
}

//#endregion

//#region

const enum AsyncIterableSourceState {
	Initial,
	DoneOK,
	DoneError,
}

/**
 * An object that allows to emit async values asynchronously or bring the iterable to an error state using `reject()`.
 * This emitter is valid only for the duration of the executor (until the promise returned by the executor settles).
 */
export interface AsyncIterableEmitter<T> {
	/**
	 * The value will be appended at the end.
	 *
	 * **NOTE** If `reject()` has already been called, this method has no effect.
	 */
	emitOne(value: T): void;
	/**
	 * The values will be appended at the end.
	 *
	 * **NOTE** If `reject()` has already been called, this method has no effect.
	 */
	emitMany(values: T[]): void;
	/**
	 * Writing an error will permanently invalidate this iterable.
	 * The current users will receive an error thrown, as will all future users.
	 *
	 * **NOTE** If `reject()` have already been called, this method has no effect.
	 */
	reject(error: Error): void;
}

/**
 * An executor for the `AsyncIterableObject` that has access to an emitter.
 */
export interface AsyncIterableExecutor<T> {
	/**
	 * @param emitter An object that allows to emit async values valid only for the duration of the executor.
	 */
	(emitter: AsyncIterableEmitter<T>): void | Promise<void>;
}

/**
 * A rich implementation for an `AsyncIterable<T>`.
 */
export class AsyncIterableObject<T> implements AsyncIterable<T> {

	public static fromArray<T>(items: T[]): AsyncIterableObject<T> { … 5 line(s) … ⟦tj:d9c23a024de84995068354cb0dedfc07⟧ }

	public static fromPromise<T>(promise: Promise<T[]>): AsyncIterableObject<T> { … 5 line(s) … ⟦tj:ccdd9b379661fcee1d5b365499bb39df⟧ }

	public static fromPromisesResolveOrder<T>(promises: Promise<T>[]): AsyncIterableObject<T> { … 5 line(s) … ⟦tj:53358e0ffe9c51ae7983e65e7e9af5f0⟧ }

	public static merge<T>(iterables: AsyncIterable<T>[]): AsyncIterableObject<T> { … 9 line(s) … ⟦tj:a0c0bf5017ef534b16c90be9f531ab9e⟧ }

	public static EMPTY = AsyncIterableObject.fromArray<any>([]);

	private _state: AsyncIterableSourceState;
	private _results: T[];
	private _error: Error | null;
	private readonly _onReturn?: () => void | Promise<void>;
	private readonly _onStateChanged: Emitter<void>;

	constructor(executor: AsyncIterableExecutor<T>, onReturn?: () => void | Promise<void>) { … 25 line(s) … ⟦tj:c59599b0b7b537c6db869b1b343f229c⟧ }

	[Symbol.asyncIterator](): AsyncIterator<T, undefined, undefined> { … 23 line(s) … ⟦tj:a62ad76400221a80df953904266deaa1⟧ }

	public static map<T, R>(iterable: AsyncIterable<T>, mapFn: (item: T) => R): AsyncIterableObject<R> { … 7 line(s) … ⟦tj:4398f8d8bdb8ff82145ea92157e21af9⟧ }

	public map<R>(mapFn: (item: T) => R): AsyncIterableObject<R> {
		return AsyncIterableObject.map(this, mapFn);
	}

	public static filter<T>(iterable: AsyncIterable<T>, filterFn: (item: T) => boolean): AsyncIterableObject<T> { … 9 line(s) … ⟦tj:4007b6130aaa300e4e15452cc43e37f6⟧ }

	public filter(filterFn: (item: T) => boolean): AsyncIterableObject<T> {
		return AsyncIterableObject.filter(this, filterFn);
	}

	public static coalesce<T>(iterable: AsyncIterable<T | undefined | null>): AsyncIterableObject<T> {
		return <AsyncIterableObject<T>>AsyncIterableObject.filter(iterable, item => !!item);
	}

	public coalesce(): AsyncIterableObject<NonNullable<T>> {
		return AsyncIterableObject.coalesce(this) as AsyncIterableObject<NonNullable<T>>;
	}

	public static async toPromise<T>(iterable: AsyncIterable<T>): Promise<T[]> { … 7 line(s) … ⟦tj:26c7a4d6b14879fda20309a35c9f5047⟧ }

	public toPromise(): Promise<T[]> {
		return AsyncIterableObject.toPromise(this);
	}

	/**
	 * The value will be appended at the end.
	 *
	 * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
	 */
	private emitOne(value: T): void { … 9 line(s) … ⟦tj:591d9099964c11ca2ea7a4fc4a26f1bd⟧ }

	/**
	 * The values will be appended at the end.
	 *
	 * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
	 */
	private emitMany(values: T[]): void { … 9 line(s) … ⟦tj:771a6b740125394c593e1b34f20278de⟧ }

	/**
	 * Calling `resolve()` will mark the result array as complete.
	 *
	 * **NOTE** `resolve()` must be called, otherwise all consumers of this iterable will hang indefinitely, similar to a non-resolved promise.
	 * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
	 */
	private resolve(): void { … 7 line(s) … ⟦tj:38920e5a5233653e7106cfda3fef24aa⟧ }

	/**
	 * Writing an error will permanently invalidate this iterable.
	 * The current users will receive an error thrown, as will all future users.
	 *
	 * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
	 */
	private reject(error: Error) { … 8 line(s) … ⟦tj:d778b81bf719a2f07d9923af09447eb9⟧ }
}

export class CancelableAsyncIterableObject<T> extends AsyncIterableObject<T> {
	constructor(
		private readonly _source: CancellationTokenSource,
		executor: AsyncIterableExecutor<T>
	) {
		super(executor);
	}

	cancel(): void {
		this._source.cancel();
	}
}

export function createCancelableAsyncIterable<T>(callback: (token: CancellationToken) => AsyncIterable<T>): CancelableAsyncIterableObject<T> { … 27 line(s) … ⟦tj:5850292081544a1b9c5be9e0c7dd621b⟧ }

export class AsyncIterableSource<T> {

	private readonly _deferred = new DeferredPromise<void>();
	private readonly _asyncIterable: AsyncIterableObject<T>;

	private _errorFn: (error: Error) => void;
	private _emitFn: (item: T) => void;

	/**
	 *
	 * @param onReturn A function that will be called when consuming the async iterable
	 * has finished by the consumer, e.g the for-await-loop has be existed (break, return) early.
	 * This is NOT called when resolving this source by its owner.
	 */
	constructor(onReturn?: () => Promise<void> | void) { … 30 line(s) … ⟦tj:f7517fd3bfb77b9e2912c47ea081048c⟧ }

	get asyncIterable(): AsyncIterableObject<T> {
		return this._asyncIterable;
	}

	resolve(): void {
		this._deferred.complete();
	}

	reject(error: Error): void { … 4 line(s) … ⟦tj:ba13b789b7f29f8e403912ba0d7e41a1⟧ }

	emitOne(item: T): void {
		this._emitFn(item);
	}
}

//#endregion
[omitted blocks are individually retrievable: call tinyjuice_retrieve with the token inside an omission marker to expand just that block]

[compacted tool output — this is a PARTIAL view; the full original (56377 bytes) is available by calling tinyjuice_retrieve with token "7d8dd8ba76dfffd242a387d3226e20de" (marker ⟦tj:7d8dd8ba76dfffd242a387d3226e20de⟧)]