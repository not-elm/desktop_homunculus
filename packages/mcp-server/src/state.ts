import { Vrm, HomunculusApiError } from "@hmcs/sdk";

export class HomunculusMcpState {
  activeCharacterEntity: number | null = null;
  private _openWebviews: number[] = [];

  get openWebviews(): number[] {
    return [...this._openWebviews];
  }

  trackWebview(entity: number): void {
    this._openWebviews.push(entity);
  }

  untrackWebview(entity: number): void {
    this._openWebviews = this._openWebviews.filter((e) => e !== entity);
  }

  lastWebview(): number | null {
    if (this._openWebviews.length === 0) {
      return null;
    }
    return this._openWebviews[this._openWebviews.length - 1];
  }

  clearWebviews(): void {
    this._openWebviews = [];
  }

  async resolveCharacter(): Promise<number> {
    if (this.activeCharacterEntity !== null) {
      try {
        await new Vrm(this.activeCharacterEntity).name();
        return this.activeCharacterEntity;
      } catch (error) {
        if (error instanceof HomunculusApiError && error.statusCode === 404) {
          this.activeCharacterEntity = null;
        } else {
          throw error;
        }
      }
    }
    const snapshots = await Vrm.findAllDetailed();
    if (snapshots.length === 0) {
      throw new Error("No VRM characters loaded in Desktop Homunculus");
    }
    this.activeCharacterEntity = snapshots[0].entity;
    return this.activeCharacterEntity;
  }
}
